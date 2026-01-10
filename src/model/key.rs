use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Key {
    bytes: [u8; 32],
}

impl Key {
    pub fn new(bytes: [u8; 32]) -> Self {
        Key { bytes }
    }

    pub fn get_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}
