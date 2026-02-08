use crate::serialise::{Bytes, SerialString, SerialiseType};

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

#[macro_export]
macro_rules! impl_to_hex {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Hex {
            fn from(value: &$t) -> Self {
                let bytes = value.try_as_bytes().unwrap();
                let string = $crate::serialise::Hex::to_hex(&bytes);
                $crate::serialise::Hex::new($crate::serialise::SerialString::new(
                    $crate::serialise::SerialiseType::Hex,
                    string,
                ))
            }
        }
    };
}

impl TryFrom<&Vec<u8>> for Hex {
    type Error = ();

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Hex,
            Self::to_hex(value),
        )))
    }
}

#[macro_export]
macro_rules! impl_from_hex {
    ($t:ty) => {
        impl From<$crate::serialise::Hex> for $t {
            fn from(value: $crate::serialise::Hex) -> Self {
                let serialised = value.get_serialised();
                let hex = serialised.get_string();
                let bytes = $crate::serialise::Hex::from_hex(&hex);
                <$t>::try_from_bytes(&bytes).unwrap()
            }
        }
    };
}

impl From<Hex> for SerialString {
    fn from(value: Hex) -> Self {
        value.get_serialised()
    }
}

impl From<SerialString> for Hex {
    fn from(value: SerialString) -> Self {
        Self::new(value)
    }
}

impl TryFrom<Bytes> for Hex {
    type Error = ();

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Hex,
            Self::to_hex(&value.get_bytes()),
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_hex() {
        let test = b"this is a really good test";
        let hex = Hex::to_hex(test);
        crate::debug!("hex {hex}");
        let bytes = Hex::from_hex(&hex);
        assert_eq!(test, bytes.as_slice());
    }
}
