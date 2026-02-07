#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    KECCAK256,
    SHA256,
    KECCAK384,
    RIPEMD160,
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::KECCAK256),
            1 => Ok(Self::SHA256),
            2 => Ok(Self::KECCAK384),
            3 => Ok(Self::RIPEMD160),
            _ => Err("Invalid hash algorithm"),
        }
    }
}
