use crate::hashing::variable_size_hash::Hash384;
use crate::hashing::HashAlgorithm;
use sha3::{Digest, Keccak384 as Keccak384Impl};

pub struct Keccak384 {}

impl Keccak384 {
    pub fn from_bytes(bytes: &[u8]) -> Hash384 {
        let mut hasher = Keccak384Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes: [u8; 48] = result.into();
        Hash384::new(HashAlgorithm::KECCAK384, bytes)
    }
}
