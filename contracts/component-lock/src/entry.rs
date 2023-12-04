// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{load_cell, load_cell_capacity, load_cell_lock, load_script, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // It the original owner has participating the tx, he/she can distribute the CKB capacity anywhere.
    if QueryIter::new(load_cell_lock, Source::Input).any(|lock| lock.as_slice() == args.as_ref()) {
        return Ok(());
    }

    // Sum capacity of inputs. Only need to scan the cells in the lock script group.
    let inputs_capacity = QueryIter::new(load_cell_capacity, Source::GroupInput).sum::<u64>();

    // Attention that output cells having the same lock script does not belong to the group.
    let outputs_capacity = QueryIter::new(load_cell, Source::Output)
        .filter(|cell| cell.lock().as_slice() == script.as_slice())
        .map(|cell| cell.capacity().unpack())
        .sum::<u64>();

    // Sum of output cells has been transferred the the owner.    // Attention that output cells having the same lock script does not belong to the group.
    let unwrapped_outputs_capacity = QueryIter::new(load_cell, Source::Output)
        .filter(|cell| cell.lock().as_slice() == args.as_ref())
        .map(|cell| cell.capacity().unpack())
        .sum::<u64>();

    debug!(
        "inputs: {}, outputs: {}, unwrapped_outputs: {}",
        inputs_capacity, outputs_capacity, unwrapped_outputs_capacity
    );

    if outputs_capacity
        .checked_add(unwrapped_outputs_capacity)
        .expect("not overflow")
        >= inputs_capacity
    {
        Ok(())
    } else {
        Err(Error::BalanceError)
    }
}
