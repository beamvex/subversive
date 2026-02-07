use crate::hashing::{Hash, HashAlgorithm};
use ripemd::{Digest, Ripemd160 as Ripemd160Impl};

pub struct Ripemd160 {
    hash: Hash,
}

impl Ripemd160 {
    #[must_use]
    pub const fn new(hash: Hash) -> Self {
        Self { hash }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Ripemd160Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();

        let hash = Hash::new(HashAlgorithm::RIPEMD160, bytes);
        Self::new(hash)
    }
}

#[macro_export]
macro_rules! impl_ripemd160_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Ripemd160 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Ripemd160::from_bytes(&value.try_as_bytes().unwrap())
            }
        }
    };
}

impl From<Ripemd160> for Hash {
    fn from(value: Ripemd160) -> Self {
        value.hash
    }
}
