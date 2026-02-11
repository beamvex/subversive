//use crate::hashing::Keccak256;
//use crate::hashing::Keccak384;
//use crate::hashing::Ripemd160;
use crate::hashing::Sha256;
use crate::serialise::Bytes;
use crate::serialise::SerialiseError;
use crate::serialise::StructType;
use crate::serialise::{AsBytes, FromBytes};

use crate::hashing::HashAlgorithm;

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

    #[must_use]
    pub fn verify(&self, bytes: &[u8]) -> bool {
        match self.algorithm {
            HashAlgorithm::SHA256 => {
                let hash: Self = Sha256::from_bytes(bytes).into();
                hash.get_bytes() == self.get_bytes()
            }
            HashAlgorithm::KECCAK256 | HashAlgorithm::KECCAK384 | HashAlgorithm::RIPEMD160 => {
                //let hash: Self = Ripemd160::from_bytes(bytes).into();
                //hash.get_bytes() == self.get_bytes()
                false
            }
        }
    }
}

impl AsBytes for Hash {
    type Error = SerialiseError;
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        bytes.push(self.algorithm.try_into().unwrap());
        bytes.extend_from_slice(&self.bytes);
        Ok(bytes)
    }
}

impl FromBytes for Hash {
    type Error = SerialiseError;
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let algorithm = HashAlgorithm::try_from(bytes[0]).unwrap();
        let bytes = bytes[1..].to_vec();
        Ok(Self::new(algorithm, bytes))
    }
}

impl TryFrom<Hash> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Hash) -> Result<Self, Self::Error> {
        Ok(value.try_as_bytes().unwrap())
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::try_from_bytes(&value).unwrap())
    }
}

#[macro_export]
macro_rules! hashable {
    ($x:ty) => {
        $crate::impl_keccak256_from_as_bytes!($x);
        $crate::impl_sha256_from_as_bytes!($x);
        $crate::impl_keccak384_from_as_bytes!($x);
        $crate::impl_ripemd160_from_as_bytes!($x);
    };
}

impl TryFrom<&Hash> for Bytes {
    type Error = SerialiseError;
    fn try_from(value: &Hash) -> Result<Self, Self::Error> {
        Ok(Self::new(StructType::HASH, value.try_as_bytes().unwrap()))
    }
}

impl TryFrom<Bytes> for Hash {
    type Error = SerialiseError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self::try_from_bytes(&value.get_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    //use crate::hashing::Keccak256;
    //use crate::hashing::Keccak384;
    use crate::hashing::Sha256;
    use crate::serialise::Base36;
    use crate::serialise::SerialString;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash: Hash = Sha256::from_bytes(&bytes).into();
        let bytes: Bytes = (&hash).try_into().unwrap();

        let hash_ss: SerialString = Base36::try_from(bytes).unwrap().try_into().unwrap();
        let hash_str = hash_ss.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");

        let base36: Base36 = hash_ss.try_into().unwrap();
        let bytes: Bytes = base36.try_into().unwrap();
        let hash: Hash = bytes.try_into().unwrap();
        crate::debug!("hash debug: {hash:?}");

        /*
        let hash_str: SerialString = Base36::try_from(Bytes::try_from(&hash).unwrap())
            .unwrap()
            .into();
        let hash_str = hash_str.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");

        let hash: Hash = Keccak384::from_bytes(&bytes).into();

        let hash_str: SerialString = Base36::try_from(Bytes::try_from(&hash).unwrap())
            .unwrap()
            .into();
        let hash_str = hash_str.get_string();
        crate::debug!("hash: {hash_str}");
        crate::debug!("hash debug: {hash:?}");
        */
    }
}
