// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, core::ScriptHashType, prelude::*},
    debug,
    error::SysError,
    high_level::{exec_cell, load_cell_data, load_script, look_for_dep_with_hash2},
};

use crate::error::Error;

use base64::{engine::general_purpose as base64_engines, Engine as _};
use ckb_ecs_schemas::ComponentDefinition;

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

fn exec_dep_cell(index: usize) -> Result<(), Error> {
    let definition = match load_cell_data(index, Source::CellDep) {
        Ok(data) => ComponentDefinition::from_slice(data.as_ref())
            .map_err(|_| Error::InvalidComponentDefinition),
        Err(SysError::IndexOutOfBound) => Err(Error::ComponentDefinitionNotFound),
        Err(err) => Err(err.into()),
    }?;

    use ckb_ecs_schemas::ComponentDefinitionUnion::*;
    let delegate = match definition.to_enum() {
        ComponentDefinitionV1(v1) => v1.delegate(),
    };
    let mut args = base64_engines::STANDARD_NO_PAD
        .encode(&delegate.args().raw_data()[..])
        .into_bytes();
    args.push(0);
    debug!("exec delegate");
    exec_cell(
        delegate.code_hash().as_slice(),
        type_or_data(delegate.hash_type().into()),
        &[core::ffi::CStr::from_bytes_with_nul(&args).expect("base64 to cstr")],
    )?;
    Ok(())
}
