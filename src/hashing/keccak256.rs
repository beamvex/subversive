use crate::hashing::{Hash, HashAlgorithm};
use sha3::{Digest, Keccak256 as Keccak256Impl};

pub struct Keccak256 {
    hash: Hash,
}

impl Keccak256 {
    pub fn new(hash: Hash) -> Self {
        Self { hash }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();
        Keccak256::new(Hash::new(HashAlgorithm::KECCAK256, bytes))
    }
}

#[macro_export]
macro_rules! impl_keccak256_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Keccak256 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Keccak256::from_bytes(value.as_bytes())
            }
        }
    };
}
