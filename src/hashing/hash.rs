use crate::hashing::{HashAlgorithm, Keccak256, Keccak384, Sha256};

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

            _ => panic!("Unknown hash algorithm"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::SerialString;
    use crate::serialise::SerialiseType;

    #[test]
    fn test_hash() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::KECCAK256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);

        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::SHA256);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);

        let hash = Hash::hash_bytes(&bytes, HashAlgorithm::KECCAK384);

        let hash_str: SerialString = hash.into_serial_string(SerialiseType::Base36);
        println!("hash: {}", hash_str.get_string());
        println!("hash debug: {:?}", hash);
    }
}
