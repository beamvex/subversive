#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    KECCAK256,
    SHA256,
    KECCAK384,
}

impl From<u8> for HashAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            0 => HashAlgorithm::KECCAK256,
            1 => HashAlgorithm::SHA256,
            2 => HashAlgorithm::KECCAK384,
            _ => panic!("Invalid hash algorithm"),
        }
    }
}
