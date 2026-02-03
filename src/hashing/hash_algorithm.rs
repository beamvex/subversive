#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    KECCAK256,
    SHA256,
    KECCAK384,
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HashAlgorithm::KECCAK256),
            1 => Ok(HashAlgorithm::SHA256),
            2 => Ok(HashAlgorithm::KECCAK384),
            _ => Err("Invalid hash algorithm"),
        }
    }
}
