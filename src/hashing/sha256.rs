use crate::hashing::Hash;
use sha2::{Digest, Sha256 as Sha256Impl};

pub struct Sha256 {}

impl Sha256 {
    pub fn from_bytes(bytes: &[u8]) -> Hash {
        let mut hasher = Sha256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.into();
        Hash::new(bytes)
    }
}
