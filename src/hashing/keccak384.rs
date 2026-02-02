use crate::hashing::{Hash, HashAlgorithm};
use sha3::{Digest, Keccak384 as Keccak384Impl};

pub struct Keccak384 {
    hash: Hash,
}

impl Keccak384 {
    pub fn new(hash: Hash) -> Self {
        Self { hash }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak384Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();
        Keccak384::new(Hash::new(HashAlgorithm::KECCAK384, bytes))
    }
}

#[macro_export]
macro_rules! impl_keccak384_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Keccak384 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Keccak384::from_bytes(value.as_bytes())
            }
        }
    };
}
