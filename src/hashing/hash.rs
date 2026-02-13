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
        let algorithm: Result<u8, SerialiseError> = self.algorithm.try_into();
        match algorithm {
            Err(error) => return Err(error),
            Ok(algorithm) => bytes.push(algorithm),
        }
        bytes.extend_from_slice(&self.bytes);
        Ok(bytes)
    }
}

impl FromBytes for Hash {
    type Error = SerialiseError;
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let algorithm = HashAlgorithm::try_from(bytes[0]);
        match algorithm {
            Err(error) => Err(error),
            Ok(algorithm) => {
                let bytes = bytes[1..].to_vec();
                Ok(Self::new(algorithm, bytes))
            }
        }
    }
}

impl TryFrom<Hash> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Hash) -> Result<Self, Self::Error> {
        match value.try_as_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(error),
        }
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Self::try_from_bytes(&value) {
            Ok(hash) => Ok(hash),
            Err(error) => Err(error),
        }
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
        match value.try_as_bytes() {
            Ok(bytes) => Ok(Self::new(StructType::HASH, bytes)),
            Err(error) => Err(error),
        }
    }
}

impl TryFrom<Bytes> for Hash {
    type Error = SerialiseError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match Self::try_from_bytes(&value.get_bytes()) {
            Ok(hash) => Ok(hash),
            Err(err) => Err(err),
        }
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
        match Bytes::try_from(&hash) {
            Ok(bytes) => match bytes.try_into_serialstring_base36() {
                Ok(hash_ss) => {
                    let hash_str = hash_ss.get_string();
                    crate::debug!("hash: {hash_str}");
                    crate::debug!("hash debug: {hash:?}");

                    match Base36::try_from(hash_ss) {
                        Ok(base36) => match Bytes::try_from(base36) {
                            Ok(bytes) => match Hash::try_from(bytes) {
                                Ok(hash) => crate::debug!("hash debug: {hash:?}"),
                                Err(error) => crate::debug!("hash error: {error:?}"),
                            },
                            Err(error) => crate::debug!("bytes error: {error:?}"),
                        },
                        Err(error) => crate::debug!("base36 error: {error:?}"),
                    }
                }
                Err(error) => crate::debug!("serialstring error: {error:?}"),
            },
            Err(error) => crate::debug!("bytes error: {error:?}"),
        }
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
