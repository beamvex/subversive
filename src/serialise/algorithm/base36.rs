use crate::serialise::{SerialString, SerialiseType};

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Base36 {
    serialised: SerialString,
}

impl Base36 {
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
    pub fn to_base36(bytes: &[u8]) -> String {
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
                *b = u8::try_from(v / 36).expect("base36 division quotient must fit in u8");
                rem = v % 36;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        out.into_iter().map(char::from).collect()
    }

    fn base36_to_bytes(base36: &str) -> Vec<u8> {
        let s = base36.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let c = c.to_ascii_lowercase();
            let digit = ALPHABET
                .iter()
                .position(|&b| b == c)
                .map(|p| u32::try_from(p).expect("base36 digit index must fit in u32"))
                .expect("invalid base36 character");

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = u32::from(*b) * 36 + carry;
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
    /// Panics if `base36` contains a character outside the base36 alphabet.
    ///
    /// Panics if the decoded value requires more than `size` bytes when `size > 0`.
    #[must_use]
    pub fn from_base36(base36: &str, size: usize) -> Vec<u8> {
        let mut bytes = Self::base36_to_bytes(base36);

        assert!(
            !(bytes.len() > size && size > 0),
            "base36 value does not fit in {size} bytes"
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
macro_rules! impl_to_base36 {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Base36 {
            fn from(value: &$t) -> Self {
                let bytes = value.try_as_bytes().unwrap();
                let string = $crate::serialise::Base36::to_base36(&bytes);
                $crate::serialise::Base36::new($crate::serialise::SerialString::new(
                    $crate::serialise::SerialiseType::Base36,
                    string,
                ))
            }
        }
    };
}

impl TryFrom<&Vec<u8>> for Base36 {
    type Error = ();

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Base36,
            Self::to_base36(value),
        )))
    }
}

#[macro_export]
macro_rules! impl_from_base36 {
    ($t:ty) => {
        impl From<$crate::serialise::Base36> for $t {
            fn from(value: $crate::serialise::Base36) -> Self {
                let serialised = value.get_serialised();
                let base36 = serialised.get_string();
                let bytes = $crate::serialise::Base36::from_base36(&base36, 0);
                <$t>::try_from_bytes(&bytes).unwrap()
            }
        }
    };
}

impl From<Base36> for SerialString {
    fn from(value: Base36) -> Self {
        value.get_serialised()
    }
}

impl From<SerialString> for Base36 {
    fn from(value: SerialString) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_base36() {
        let test = b"this is a really good test";
        let base36 = Base36::to_base36(test);
        crate::debug!("base36 {base36}");
        let bytes = Base36::from_base36(&base36, 0);
        assert_eq!(test, bytes.as_slice());
    }
}
