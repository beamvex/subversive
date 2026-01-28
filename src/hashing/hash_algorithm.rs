use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, AsBytes, Unaligned, FromZeroes, FromBytes)]
pub struct HashAlgorithm(u8);

impl HashAlgorithm {
    pub const KECCAK256: HashAlgorithm = HashAlgorithm(0);
    pub const SHA256: HashAlgorithm = HashAlgorithm(1);
    pub const KECCAK384: HashAlgorithm = HashAlgorithm(2);
}
