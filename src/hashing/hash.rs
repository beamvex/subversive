use crate::serialise::{AsBytes, FromBytes};

use crate::{hashing::HashAlgorithm, serialisable};

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
        $crate::impl_keccak256_from_as_bytes!($x);
        $crate::impl_sha256_from_as_bytes!($x);
        $crate::impl_keccak384_from_as_bytes!($x);
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::hashing::Keccak256;
    use crate::hashing::Keccak384;
    use crate::hashing::Sha256;
    use crate::serialise::Base36;
    use crate::serialise::SerialString;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash: Hash = Keccak256::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::from(&hash).into();

        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);

        let hash: Hash = Sha256::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::from(&hash).into();

        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);

        let hash: Hash = Keccak384::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::from(&hash).into();
        crate::debug!("hash: {}", hash_str.get_string());
        crate::debug!("hash debug: {:?}", hash);
    }
}
