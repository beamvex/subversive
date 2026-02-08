use crate::serialise::{Bytes, SerialString, SerialiseType};

const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Base58 {
    serialised: SerialString,
}

impl Base58 {
    #[must_use]
    pub const fn new(serialised: SerialString) -> Self {
        Self { serialised }
    }

    #[must_use]
    pub fn get_serialised(self) -> SerialString {
        self.serialised
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_base58(bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "0".to_string();
        }

        if bytes.iter().all(|&b| b == 0) {
            return "0".to_string();
        }

        let mut n = bytes.to_vec();
        let mut out: Vec<u8> = Vec::new();

        while !n.is_empty() && n.iter().any(|&b| b != 0) {
            let mut rem: u32 = 0;
            for b in &mut n {
                let v = (rem << 8) | u32::from(*b);
                *b = u8::try_from(v / 58).expect("base58 division quotient must fit in u8");
                rem = v % 58;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        out.into_iter().map(char::from).collect()
    }

    fn base58_to_bytes(base58: &str) -> Vec<u8> {
        let s = base58.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let digit = ALPHABET
                .iter()
                .position(|&b| b == c)
                .map(|p| u32::try_from(p).expect("base58 digit index must fit in u32"))
                .expect("invalid base58 character");

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = u32::from(*b) * 58 + carry;
                *b = (v & 0xff) as u8;
                carry = v >> 8;
            }

            while carry > 0 {
                bytes.insert(0, (carry & 0xff) as u8);
                carry >>= 8;
            }
        }

        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }

        bytes
    }

    /// Decodes a base36 string into bytes, optionally left-padding to `size`.
    ///
    /// # Panics
    ///
    /// Panics if `base58` contains a character outside the base58 alphabet.
    ///
    /// Panics if the decoded value requires more than `size` bytes when `size > 0`.
    #[must_use]
    pub fn from_base58(base58: &str, size: usize) -> Vec<u8> {
        let mut bytes = Self::base58_to_bytes(base58);

        assert!(
            !(bytes.len() > size && size > 0),
            "base58 value does not fit in {size} bytes"
        );

        if bytes.len() < size && size > 0 {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

#[macro_export]
macro_rules! impl_to_base58 {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Base58 {
            fn from(value: &$t) -> Self {
                let bytes = value.try_as_bytes().unwrap();
                let string = $crate::serialise::Base58::to_base58(&bytes);
                $crate::serialise::Base58::new($crate::serialise::SerialString::new(
                    $crate::serialise::SerialiseType::Base58,
                    string,
                ))
            }
        }
    };
}

impl TryFrom<&Vec<u8>> for Base58 {
    type Error = ();

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Base58,
            Self::to_base58(value),
        )))
    }
}

#[macro_export]
macro_rules! impl_from_base58 {
    ($t:ty) => {
        impl From<$crate::serialise::Base58> for $t {
            fn from(value: $crate::serialise::Base58) -> Self {
                let serialised = value.get_serialised();
                let base58 = serialised.get_string();
                let bytes = $crate::serialise::Base58::from_base58(&base58, 0);
                <$t>::try_from_bytes(&bytes).unwrap()
            }
        }
    };
}

impl From<Base58> for SerialString {
    fn from(value: Base58) -> Self {
        value.get_serialised()
    }
}

impl From<SerialString> for Base58 {
    fn from(value: SerialString) -> Self {
        Self::new(value)
    }
}

impl TryFrom<Bytes> for Base58 {
    type Error = ();

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Base58,
            Self::to_base58(&value.get_bytes()),
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_base58() {
        let test = b"this is a really good test";
        let base58 = Base58::to_base58(test);
        crate::debug!("base58 {base58}");
        let bytes = Base58::from_base58(&base58, 0);
        assert_eq!(test, bytes.as_slice());
    }
}
