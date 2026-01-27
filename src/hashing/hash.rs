use crate::{
    hashing::{HashAlgorithm, Keccak256, Sha256},
    serialise,
};

use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Hash {
    algorithm: HashAlgorithm,
    bytes: [u8; 32],
}

impl Hash {
    pub fn new(algorithm: HashAlgorithm, bytes: [u8; 32]) -> Self {
        Hash { algorithm, bytes }
    }

    fn from(bytes: &[u8], hash_algorithm: HashAlgorithm) -> Self {
        match hash_algorithm {
            HashAlgorithm::KECCAK256 => Keccak256::from_bytes(bytes),
            HashAlgorithm::SHA256 => Sha256::from_bytes(bytes),
            _ => panic!("Unknown hash algorithm"),
        }
    }
}

serialise!(Hash);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::SerialString;
    use crate::serialise::SerialiseType;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash = Hash::from(&bytes, HashAlgorithm::KECCAK256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);

        let hash = Hash::from(&bytes, HashAlgorithm::SHA256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);
    }
}
