use crate::hashing::{Hash, HashAlgorithm};
use sha3::{Digest, Keccak256 as Keccak256Impl};

pub struct Keccak256 {}

impl Keccak256 {
    pub fn from_bytes(bytes: &[u8]) -> Hash {
        let mut hasher = Keccak256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();
        Hash::new(HashAlgorithm::KECCAK256, bytes)
    }
}
