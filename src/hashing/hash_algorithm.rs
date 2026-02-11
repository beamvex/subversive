use crate::serialise::SerialiseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    KECCAK256,
    SHA256,
    KECCAK384,
    RIPEMD160,
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = SerialiseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::KECCAK256),
            101 => Ok(Self::SHA256),
            102 => Ok(Self::KECCAK384),
            103 => Ok(Self::RIPEMD160),
            _ => Err(SerialiseError::new("Invalid hash algorithm".to_string())),
        }
    }
}

impl TryFrom<HashAlgorithm> for u8 {
    type Error = SerialiseError;
    fn try_from(value: HashAlgorithm) -> Result<Self, Self::Error> {
        match value {
            HashAlgorithm::KECCAK256 => Ok(100),
            HashAlgorithm::SHA256 => Ok(101),
            HashAlgorithm::KECCAK384 => Ok(102),
            HashAlgorithm::RIPEMD160 => Ok(103),
        }
    }
}
