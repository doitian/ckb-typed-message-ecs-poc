use crate::schemas::{Byte32, Bytes, String};
use molecule::prelude::*;

impl From<[u8; 32]> for Byte32 {
    fn from(value: [u8; 32]) -> Self {
        Byte32::from_slice(&value).expect("Byte32 from [u8; 32]")
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        let len = (value.len() as u32).to_le_bytes();
        let mut v = Vec::with_capacity(4 + value.len());
        v.extend_from_slice(&len[..]);
        v.extend_from_slice(value);
        Bytes::new_unchecked(v.into())
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        let len = (bytes.len() as u32).to_le_bytes();
        let mut v = Vec::with_capacity(4 + bytes.len());
        v.extend_from_slice(&len[..]);
        v.extend_from_slice(bytes);
        String::new_unchecked(v.into())
    }
}
