use crate::{
    serialisable,
    serialise::{SerialString, SerialiseError, SerialiseType},
    string_serialisable,
};

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// Base36 encoding implementation (0-9 and A-Z).
///
/// This type provides methods to encode and decode data using base36 encoding,
/// which uses the digits 0-9 and letters A-Z to represent data.
#[derive(Debug)]
pub struct Base36 {
    /// The base36-encoded string representation
    serialised: SerialString,
}

impl Base36 {
    /// Creates a new `Base36` instance.
    ///
    /// # Arguments
    /// * `serialised` - The base36-encoded string
    #[must_use = "This creates a new Base36 instance but does nothing if unused"]
    pub const fn new(serialised: SerialString) -> Self {
        Self { serialised }
    }

    /// Returns the base36-encoded string.
    #[must_use = "This returns the encoded string but does nothing if unused"]
    pub fn get_serialised(self) -> SerialString {
        self.serialised
    }

    /// Attempts to convert this value into a base36-encoded string.
    ///
    /// # Returns
    /// The base36-encoded string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if the conversion fails
    pub fn try_into_serialstring_base36(self) -> Result<SerialString, SerialiseError> {
        Ok(self.serialised)
    }

    /// Attempts to create a `Base36` instance from a serialized string.
    ///
    /// # Arguments
    /// * `serial_string` - The serialized string to parse
    ///
    /// # Returns
    /// A new `Base36` instance
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The serialization type is not Base36
    /// - The string cannot be decoded as base36
    pub fn try_from_serial_string(serial_string: SerialString) -> Result<Self, SerialiseError> {
        if serial_string.get_serialise_type() != SerialiseType::Base36 {
            return Err(SerialiseError::new("Invalid SerialiseType".into()));
        }
        Ok(Self::new(serial_string))
    }

    /// Encodes a byte slice using base36 encoding.
    ///
    /// # Arguments
    /// * `bytes` - The bytes to encode
    ///
    /// # Returns
    /// The base36-encoded string
    #[must_use = "This returns the encoded string and does nothing if unused"]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_base36(bytes: &[u8]) -> String {
        if bytes.is_empty() || bytes.iter().all(|&b| b == 0) {
            return "0".to_string();
        }

        let mut n: Vec<u8> = bytes.to_vec();
        let mut out = Vec::new();

        while !n.is_empty() {
            let mut rem = 0;
            let mut i = 0;

            while i < n.len() {
                let v = u32::from(n[i]) + (rem * 256);
                n[i] = (v / 36) as u8;
                rem = v % 36;
                i += 1;
            }

            out.push(rem as u8);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        let mut result = String::with_capacity(out.len());
        for &byte in out.iter().rev() {
            result.push(ALPHABET[byte as usize] as char);
        }
        result
    }

    fn base36_to_bytes(base36: &str) -> Vec<u8> {
        let s = base36.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes = Vec::new();
        for c in s.chars() {
            let digit = ALPHABET
                .iter()
                .position(|&x| x == c.to_ascii_lowercase() as u8)
                .expect("invalid base36 character");
            bytes.push(digit as u8);
        }

        let mut carry = 0;
        for b in bytes.iter_mut().rev() {
            let v = u32::from(*b) * 36 + carry;
            *b = (v & 0xff) as u8;
            carry = v >> 8;
        }

        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
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
    /// Decodes a base36 string into bytes.
    ///
    /// # Arguments
    /// * `base36` - The base36-encoded string to decode
    /// * `size` - The expected size of the output in bytes. If greater than 0,
    ///   the output will be padded or truncated to this size.
    ///
    /// # Returns
    /// The decoded bytes
    ///
    /// # Panics
    /// * If `base36` contains characters outside the base36 alphabet
    /// * If the decoded value requires more than `size` bytes when `size > 0`
    #[must_use = "This returns the decoded bytes and does nothing if unused"]
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

impl TryFrom<Base36> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Base36) -> Result<Self, Self::Error> {
        Ok(Base36::from_base36(value.get_serialised().get_string(), 0))
    }
}

impl TryFrom<Vec<u8>> for Base36 {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::Base36,
            Self::to_base36(&value),
        )))
    }
}

serialisable!(Base36);
string_serialisable!(Base36);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::Bytes;
    use crate::serialise::SerialString;

    use crate::serialise::StructType;

    #[test]
    pub fn test_base36() {
        let test = b"this is a really good test";
        let test_bytes = Bytes::new(StructType::HASH, test.to_vec());
        let test_bytes2 = test_bytes.clone();
        let base36_result: Result<Base36, SerialiseError> = test_bytes.try_into();

        if let Ok(base36) = base36_result {
            slogger::debug!("base36 {base36:?}");

            let serialised: Result<SerialString, SerialiseError> =
                base36.try_into_serialstring_base36();
            if let Ok(serialised) = serialised {
                let serialised_str = serialised.get_string();
                slogger::debug!("test_bytes_restored {serialised_str}");

                let deserialised: Result<Base36, SerialiseError> =
                    Base36::try_from_serial_string(serialised);
                if let Ok(deserialised) = deserialised {
                    let test_bytes_restored: Result<Bytes, SerialiseError> =
                        deserialised.try_into();
                    match test_bytes_restored {
                        Err(e) => slogger::debug!("test_bytes_restored error {e:?}"),
                        Ok(test_bytes_restored) => {
                            assert_eq!(test, test_bytes_restored.get_bytes().as_slice());
                        }
                    }
                }
            }
        }

        let mut badvec: Vec<u8> = vec![];
        badvec.push(99);
        badvec.extend_from_slice(test_bytes2.get_bytes().as_slice());

        let base36_str = Base36::to_base36(&badvec);
        let base36 = Base36::new(SerialString::new(SerialiseType::Base36, base36_str));
        slogger::debug!("base36 {base36:?}");

        let serialised: Result<SerialString, SerialiseError> =
            base36.try_into_serialstring_base36();
        if let Ok(serialised) = serialised {
            let serialised_str = serialised.get_string();
            slogger::debug!("test_bytes_restored {serialised_str}");

            let deserialised: Result<Base36, SerialiseError> =
                Base36::try_from_serial_string(serialised);
            if let Ok(deserialised) = deserialised {
                let test_bytes_restored: Result<Bytes, SerialiseError> = deserialised.try_into();
                assert!(test_bytes_restored.is_err());
            }
        }
    }

    #[test]
    pub fn test_invalid_base36() {
        let test = b"this is a failure test; its a little bit manufactured as this shouldnt be possible via code";
        let test_bytes = test.to_vec();
        let mut badvec: Vec<u8> = vec![];
        badvec.push(99);
        badvec.extend_from_slice(&test_bytes);

        let base36: Base36 = Base36::new(SerialString::new(
            SerialiseType::Base36,
            Base36::to_base36(&badvec),
        ));
        slogger::debug!("base36 {base36:?}");

        let serialised: Result<SerialString, SerialiseError> =
            base36.try_into_serialstring_base36();
        if let Ok(serialised) = serialised {
            let serialised_str = serialised.get_string();
            slogger::debug!("test_bytes_restored {serialised_str}");

            let deserialised: Result<Base36, SerialiseError> =
                Base36::try_from_serial_string(serialised);
            if let Ok(deserialised) = deserialised {
                let test_bytes_restored: Result<Bytes, SerialiseError> = deserialised.try_into();
                assert!(test_bytes_restored.is_err());
            }
        }
    }
}
