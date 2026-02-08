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
            100 => Ok(Self::STRING),
            101 => Ok(Self::HASH),
            102 => Ok(Self::KEY),
            103 => Ok(Self::ADDRESS),
            104 => Ok(Self::SIGNATURE),
            _ => Err("Invalid struct type"),
        }
    }
}

impl TryFrom<StructType> for u8 {
    type Error = &'static str;
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
