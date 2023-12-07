// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_types::{bytes::Bytes, core::ScriptHashType, prelude::*},
    error::SysError,
    high_level::{load_script, look_for_dep_with_hash2},
};

use crate::error::Error;

const ARGS_LEN: usize = 33;
const CODE_HASH_LEN: usize = 32;
const SCRIPT_HASH_TYPE_POS: usize = 32;

// Only need to differentiate type and data, no mather which vm version for data.
fn type_or_data(byte: u8) -> ScriptHashType {
    if byte == ScriptHashType::Type as u8 {
        ScriptHashType::Type
    } else {
        ScriptHashType::Data
    }
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    if args.len() < ARGS_LEN {
        return Err(Error::InvalidArgs);
    }

    match look_for_dep_with_hash2(
        &args[0..CODE_HASH_LEN],
        type_or_data(args[SCRIPT_HASH_TYPE_POS]),
    ) {
        Ok(index) => exec_dep_cell(index),
        Err(SysError::IndexOutOfBound) => Err(Error::ComponentDefinitionNotFound),
        Err(err) => Err(err.into()),
    }
}

fn exec_dep_cell(_: usize) -> Result<(), Error> {
    Ok(())
}
