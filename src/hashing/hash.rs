use crate::{
    hashing::{HashAlgorithm, Keccak256, Sha256},
    serialise,
};

use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Hash {
    bytes: [u8; 32],
}

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Hash { bytes }
    }

    fn from(bytes: &[u8], hash_algorithm: HashAlgorithm) -> Self {
        match hash_algorithm {
            HashAlgorithm::Keccak256 => Keccak256::from_bytes(bytes),
            HashAlgorithm::Sha256 => Sha256::from_bytes(bytes),
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
        let hash = Hash::from(&bytes, HashAlgorithm::Keccak256);

        let hash: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash.get_string());

        let hash = Hash::from(&bytes, HashAlgorithm::Sha256);

        let hash: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash.get_string());
    }
}
