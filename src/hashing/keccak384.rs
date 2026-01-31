use crate::hashing::Hash;
use crate::hashing::HashAlgorithm;
use sha3::{Digest, Keccak384 as Keccak384Impl};

pub struct Keccak384 {}

impl Keccak384 {
    pub fn from_bytes(bytes: &[u8]) -> Hash {
        let mut hasher = Keccak384Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();
        Hash::new(HashAlgorithm::KECCAK384, bytes)
    }
}
