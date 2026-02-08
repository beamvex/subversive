use crate::serialise::{Bytes, SerialString, SerialiseType};

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug)]
pub struct Base64 {
    serialised: SerialString,
}

impl Base64 {
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
    pub fn to_base64(bytes: &[u8]) -> String {
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
                *b = u8::try_from(v / 64).expect("base64 division quotient must fit in u8");
                rem = v % 64;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        out.into_iter().map(char::from).collect()
    }

    fn base64_to_bytes(base64: &str) -> Vec<u8> {
        let s = base64.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let digit = ALPHABET
                .iter()
                .position(|&b| b == c)
                .map(|p| u32::try_from(p).expect("base64 digit index must fit in u32"))
                .expect("invalid base64 character");

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = u32::from(*b) * 64 + carry;
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

    /// Decodes a base64 string into bytes, optionally left-padding to `size`.
    ///
    /// # Panics
    ///
    /// Panics if `base64` contains a character outside the base64 alphabet.
    ///
    /// Panics if the decoded value requires more than `size` bytes when `size > 0`.
    #[must_use]
    pub fn from_base64(base64: &str, size: usize) -> Vec<u8> {
        let mut bytes = Self::base64_to_bytes(base64);

        assert!(
            !(bytes.len() > size && size > 0),
            "base64 value does not fit in {size} bytes"
        );

        if bytes.len() < size && size > 0 {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

impl From<Base64> for SerialString {
    fn from(value: Base64) -> Self {
        value.get_serialised()
    }
}

impl From<SerialString> for Base64 {
    fn from(value: SerialString) -> Self {
        Self::new(value)
    }
}

impl TryFrom<Bytes> for Base64 {
    type Error = ();

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Base64,
            Self::to_base64(&value.get_bytes()),
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_base64() {
        let test = b"this is a really good test";
        let base64 = Base64::to_base64(test);
        crate::debug!("base64 {base64}");
        let bytes = Base64::from_base64(&base64, 0);
        assert_eq!(test, bytes.as_slice());
    }
}
