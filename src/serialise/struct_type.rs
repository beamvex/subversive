#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StructType {
    STRING,
    HASH,
    KEY,
    ADDRESS,
    SIGNATURE,
}

impl TryFrom<u8> for StructType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STRING),
            1 => Ok(Self::HASH),
            2 => Ok(Self::KEY),
            3 => Ok(Self::ADDRESS),
            4 => Ok(Self::SIGNATURE),
            _ => Err("Invalid struct type"),
        }
    }
}
