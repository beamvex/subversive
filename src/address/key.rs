use crate::{
    hashable, serialisable,
    serialise::{AsBytes, FromBytes},
};

pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Key { bytes }
    }
}

impl AsBytes for Key {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.bytes);
        bytes
    }
}

impl FromBytes for Key {
    fn from_bytes(bytes: &[u8]) -> Self {
        let bytes = bytes.to_vec();
        Key::from(bytes)
    }
}

serialisable!(Key);

hashable!(Key);

#[cfg(test)]
mod tests {

    use crate::serialise::{serial_string, SerialiseType};

    use super::*;

    #[test]
    fn test_from_base36() {
        let b36_string = "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833";
        let serial_string = serial_string::SerialString::new(
            crate::serialise::SerialiseType::Base36,
            b36_string.to_string(),
        );
        let key = Key::from(&serial_string);

        let key: serial_string::SerialString = key.into_serial_string(SerialiseType::Base36);
        println!("key: {}", key);

        assert_eq!(
            key.get_string(),
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833"
        );
    }
}
