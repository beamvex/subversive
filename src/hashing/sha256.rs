use crate::hashing::{Hash, HashAlgorithm};
use sha2::{Digest, Sha256 as Sha256Impl};

pub struct Sha256 {
    hash: Hash,
}

impl Sha256 {
    #[must_use]
    pub const fn new(hash: Hash) -> Self {
        Self { hash }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();

        let hash = Hash::new(HashAlgorithm::SHA256, bytes);
        Self::new(hash)
    }
}

#[macro_export]
macro_rules! impl_sha256_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Sha256 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Sha256::from_bytes(&value.try_as_bytes().unwrap())
            }
        }
    };
}

impl From<Sha256> for Hash {
    fn from(value: Sha256) -> Self {
        value.hash
    }
}
