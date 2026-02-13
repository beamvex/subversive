use crate::serialise::SerialiseError;

/// Supported cryptographic hash algorithms.
///
/// This enum represents the different hash algorithms that can be used
/// to create hash values in the system.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    /// Keccak-256 hash algorithm (used in Ethereum)
    KECCAK256,
    /// SHA-256 hash algorithm from the SHA-2 family
    SHA256,
    /// Keccak-384 hash algorithm
    KECCAK384,
    /// RIPEMD-160 hash algorithm
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
