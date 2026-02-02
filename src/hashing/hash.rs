use crate::serialise::{AsBytes, FromBytes};

use crate::{
    hashing::{HashAlgorithm, Keccak256, Keccak384, Sha256},
    serialisable,
};

#[derive(Debug)]
pub struct Hash {
    algorithm: HashAlgorithm,
    bytes: Vec<u8>,
}

impl Hash {
    pub fn new(algorithm: HashAlgorithm, bytes: Vec<u8>) -> Self {
        Hash { algorithm, bytes }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn get_algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }

    pub fn hash_bytes(bytes: &[u8], hash_algorithm: HashAlgorithm) -> Self {
        match hash_algorithm {
            HashAlgorithm::KECCAK256 => Keccak256::from_bytes(bytes),
            HashAlgorithm::SHA256 => Sha256::from_bytes(bytes),
            HashAlgorithm::KECCAK384 => Keccak384::from_bytes(bytes),
        }
    }
}

impl AsBytes for Hash {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.algorithm as u8);
        bytes.extend_from_slice(&self.bytes);
        bytes
    }
}

impl FromBytes for Hash {
    fn from_bytes(bytes: &[u8]) -> Self {
        let algorithm = HashAlgorithm::from(bytes[0]);
        let bytes = bytes[1..].to_vec();
        Hash::new(algorithm, bytes)
    }
}

serialisable!(Hash);

#[macro_export]
macro_rules! hashable {
    ($x:ty) => {
        impl $x {
            pub fn hash(
                &self,
                hash_algorithm: $crate::hashing::HashAlgorithm,
            ) -> $crate::hashing::Hash {
                $crate::hashing::Hash::hash_bytes(&self.as_bytes(), hash_algorithm)
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::Base36;
    use crate::serialise::SerialString;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::KECCAK256);

        let hash_str: SerialString = {
            let hashb36: Base36 = hash.into();
            hashb36.into()
        };
        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);

        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::SHA256);

        let hash_str: SerialString = {
            let hashb36: Base36 = hash.into();
            hashb36.into()
        };
        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);

        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::KECCAK384);

        let hash_str: SerialString = {
            let hashb36: Base36 = hash.into();
            hashb36.into()
        };
        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);
    }
}
