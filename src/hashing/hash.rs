use crate::serialise::{AsBytes, FromBytes};

use crate::{hashing::HashAlgorithm, serialisable};

#[derive(Debug)]
pub struct Hash {
    algorithm: HashAlgorithm,
    bytes: Vec<u8>,
}

impl Hash {
    #[must_use]
    pub const fn new(algorithm: HashAlgorithm, bytes: Vec<u8>) -> Self {
        Self { algorithm, bytes }
    }

    #[must_use]
    pub const fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    #[must_use]
    pub const fn get_algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }
}

impl AsBytes for Hash {
    type Error = &'static str;
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        bytes.push(self.algorithm as u8);
        bytes.extend_from_slice(&self.bytes);
        Ok(bytes)
    }
}

impl FromBytes for Hash {
    type Error = &'static str;
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let algorithm = HashAlgorithm::try_from(bytes[0]).unwrap();
        let bytes = bytes[1..].to_vec();
        Ok(Self::new(algorithm, bytes))
    }
}

serialisable!(Hash);

#[macro_export]
macro_rules! hashable {
    ($x:ty) => {
        $crate::impl_keccak256_from_as_bytes!($x);
        $crate::impl_sha256_from_as_bytes!($x);
        $crate::impl_keccak384_from_as_bytes!($x);
        $crate::impl_ripemd160_from_as_bytes!($x);
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
        let hash_str = hash_str.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");

        let hash: Hash = Sha256::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::from(&hash).into();
        let hash_str = hash_str.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");

        let hash: Hash = Keccak384::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::from(&hash).into();
        let hash_str = hash_str.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");
    }
}
