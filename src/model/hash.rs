use crate::utils::{FromBase36, ToBase36};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Hash {
    bytes: [u8; 32],
}

impl ToBase36 for Hash {}

impl FromBase36 for Hash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Hash::read_from(bytes).unwrap()
    }
}

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Hash { bytes }
    }
}
