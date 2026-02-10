use crate::{
    serialisable,
    serialise::{SerialString, SerialiseError, SerialiseType},
    string_serialisable,
};

const ALPHABET: &[u8; 16] = b"0123456789abcdef";

#[derive(Debug)]
pub struct Hex {
    serialised: SerialString,
}

impl Hex {
    #[must_use]
    pub const fn new(serialised: SerialString) -> Self {
        Self { serialised }
    }

    #[must_use]
    pub fn get_serialised(self) -> SerialString {
        self.serialised
    }

    #[must_use]
    pub fn to_hex(bytes: &[u8]) -> String {
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
        for &b in bytes {
            out.push(ALPHABET[(b >> 4) as usize]);
            out.push(ALPHABET[(b & 0x0f) as usize]);
        }

        // `out` is guaranteed to be ASCII.
        unsafe { String::from_utf8_unchecked(out) }
    }

    const fn from_hex_digit(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(10 + (c - b'a')),
            b'A'..=b'F' => Some(10 + (c - b'A')),
            _ => None,
        }
    }

    /// Decodes a hex string into bytes.
    ///
    /// # Panics
    ///
    /// Panics if `hex` contains a non-hex character.
    ///
    /// Panics if `hex` contains an odd number of characters.
    #[must_use]
    pub fn from_hex(hex: &str) -> Vec<u8> {
        let s = hex.trim();
        if s.is_empty() {
            return vec![];
        }

        assert!(
            s.len().is_multiple_of(2),
            "hex string must have an even length"
        );

        let mut out: Vec<u8> = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let hi = Self::from_hex_digit(bytes[i]).expect("invalid hex character");
            let lo = Self::from_hex_digit(bytes[i + 1]).expect("invalid hex character");
            out.push((hi << 4) | lo);
        }
        out
    }
}

impl TryFrom<Hex> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Hex) -> Result<Self, Self::Error> {
        Ok(Hex::from_hex(value.get_serialised().get_string()))
    }
}

impl TryFrom<Vec<u8>> for Hex {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Hex,
            Self::to_hex(&value),
        )))
    }
}

serialisable!(Hex);
string_serialisable!(Hex);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::Bytes;
    use crate::serialise::SerialString;
    use crate::serialise::SerialiseError;
    use crate::serialise::StructType;

    #[test]
    pub fn test_hex() {
        let test = b"this is a really good test";
        let test_bytes = Bytes::new(StructType::HASH, test.to_vec());
        let hex: Hex = test_bytes.try_into().unwrap();
        crate::debug!("hex {hex:?}");
        let serialised: SerialString = hex.try_into().unwrap();
        let serialised_str = serialised.get_string();
        crate::debug!("test_bytes_restored {serialised_str}");
        let deserialised: Hex = serialised.try_into().unwrap();
        let test_bytes_restored: Bytes = deserialised.try_into().unwrap();
        assert_eq!(test, test_bytes_restored.get_bytes().as_slice());
    }

    #[test]
    pub fn test_invalid_hex() {
        let test = b"this is a failure test; its a little bit manufactured as this shouldnt be possible via code";
        let test_bytes = test.to_vec();
        let mut badvec: Vec<u8> = vec![];
        badvec.push(99);
        badvec.extend_from_slice(&test_bytes);

        let hex: Hex = Hex::new(SerialString::new(SerialiseType::Hex, Hex::to_hex(&badvec)));
        crate::debug!("hex {hex:?}");

        let serialised: SerialString = hex.try_into().unwrap();

        let deserialised: Hex = serialised.try_into().unwrap();
        let test_bytes_restored: Result<Bytes, SerialiseError> = deserialised.try_into();

        assert!(test_bytes_restored.is_err());
    }
}
