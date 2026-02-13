use crate::serialise::SerialiseError;

/// Types of serializable structures in the system.
///
/// This enum represents the different types of data structures that can be
/// serialized and deserialized. Each variant corresponds to a specific
/// type of data with its own serialization rules.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StructType {
    /// String data type
    STRING,
    /// Cryptographic hash value
    HASH,
    /// Cryptographic key (public or private)
    KEY,
    /// Network or blockchain address
    ADDRESS,
    /// Cryptographic signature
    SIGNATURE,
}

impl TryFrom<u8> for StructType {
    type Error = SerialiseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::STRING),
            101 => Ok(Self::HASH),
            102 => Ok(Self::KEY),
            103 => Ok(Self::ADDRESS),
            104 => Ok(Self::SIGNATURE),
            _ => Err(SerialiseError::new("Invalid struct type".to_string())),
        }
    }
}

impl TryFrom<StructType> for u8 {
    type Error = SerialiseError;
    fn try_from(value: StructType) -> Result<Self, Self::Error> {
        match value {
            StructType::STRING => Ok(100),
            StructType::HASH => Ok(101),
            StructType::KEY => Ok(102),
            StructType::ADDRESS => Ok(103),
            StructType::SIGNATURE => Ok(104),
        }
    }
}
