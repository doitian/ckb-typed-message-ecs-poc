// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::*,
    debug,
    error::SysError,
    high_level::{
        load_cell_capacity, load_cell_data, load_cell_type, load_input, load_script, QueryIter,
    },
};

use blake2b_rs::{Blake2b, Blake2bBuilder};
use ckb_ecs_schemas::ComponentDefinitionReader;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    verify_type_id()?;
    verify_component_definition()
}

// https://github.com/nervosnetwork/ckb/blob/develop/script/src/type_id.rs
pub fn verify_type_id() -> Result<(), Error> {
    let script = load_script()?;

    // TYPE_ID script should only accept one argument,
    // which is the hash of all inputs when creating
    // the cell.
    if script.args().len() != 32 {
        return Err(Error::InvalidArgs);
    }

    if cell_exists(1, Source::GroupInput)? || cell_exists(1, Source::GroupOutput)? {
        return Err(Error::TooManyCells);
    }

    // If there's only one output cell with current
    // TYPE_ID script, we are creating such a cell,
    // we also need to validate that the first argument matches
    // the hash of following items concatenated:
    // 1. First CellInput of the transaction.
    // 2. Index of the first output cell in current script group.
    if !(cell_exists(0, Source::GroupInput)?) {
        let first_cell_input = load_input(0, Source::Input).expect("Tx has at least one input");
        let first_output_index = QueryIter::new(load_cell_type, Source::Output)
            .enumerate()
            .find(|(_, output_type_opt)| {
                output_type_opt
                    .as_ref()
                    .map_or(false, |s| s.as_slice() == script.as_slice())
            })
            .expect("Tx must have an output in this group")
            .0 as u64;

        let mut blake2b = new_blake2b();

        blake2b.update(first_cell_input.as_slice());
        blake2b.update(&first_output_index.to_le_bytes());
        let mut ret = [0; 32];
        blake2b.finalize(&mut ret);

        if ret[..] != script.args().raw_data()[..] {
            return Err(Error::InvalidTypeID);
        }
    }

    Ok(())
}

pub fn verify_component_definition() -> Result<(), Error> {
    // There's at most one output
    match load_cell_data(0, Source::GroupOutput) {
        Ok(data) => match ComponentDefinitionReader::from_slice(data.as_slice()) {
            Ok(_) => Ok(()),
            Err(err) => {
                debug!("ComponentDefinition verfication error: {}", err);
                Err(Error::InvalidData)
            }
        },
        Err(SysError::IndexOutOfBound) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub fn cell_exists(index: usize, source: Source) -> Result<bool, Error> {
    match load_cell_capacity(index, source) {
        Ok(_) => Ok(true),
        Err(SysError::IndexOutOfBound) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

pub const CKB_PERSONALIZATION: &[u8] = b"ckb-default-hash";
pub fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32)
        .personal(CKB_PERSONALIZATION)
        .build()
}
