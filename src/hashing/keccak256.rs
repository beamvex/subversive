use crate::hashing::{Hash, HashAlgorithm};
use sha3::{Digest, Keccak256 as Keccak256Impl};

pub struct Keccak256 {
    hash: Hash,
}

impl Keccak256 {
    #[must_use]
    pub const fn new(hash: Hash) -> Self {
        Self { hash }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();
        Self::new(Hash::new(HashAlgorithm::KECCAK256, bytes))
    }
}

#[macro_export]
macro_rules! impl_keccak256_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Keccak256 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Keccak256::from_bytes(&value.try_as_bytes().unwrap())
            }
        }
    };
}

impl From<Keccak256> for Hash {
    fn from(value: Keccak256) -> Self {
        value.hash
    }
}

#[cfg(test)]
mod tests {

    use crate::serialise::base36::Base36;
    use crate::serialise::SerialString;

    use super::*;

    #[test]
    pub fn test_ripemd160() {
        let test = b"this is a really good test";
        let hash: Hash = Keccak256::from_bytes(test).into();
        let serialised: SerialString = Base36::from(&hash).into();

        assert!(hash.verify(test));
        crate::debug!("keccak256 {serialised}");
    }
}
