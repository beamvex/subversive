use crate::{
    hashable, serialisable,
    serialise::{AsBytes, Bytes, FromBytes, StructType},
};

pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    #[must_use]
    pub const fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl AsBytes for Key {
    type Error = ();
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.bytes);
        Ok(bytes)
    }
}

impl FromBytes for Key {
    type Error = ();
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes = bytes.to_vec();
        Ok(Self::from(bytes))
    }
}

impl TryFrom<Key> for Bytes {
    type Error = &'static str;
    fn try_from(value: Key) -> Result<Self, Self::Error> {
        Ok(Self::new(StructType::KEY, value.try_as_bytes().unwrap()))
    }
}

serialisable!(Key);

hashable!(Key);

#[cfg(test)]
mod tests {

    use crate::serialise::{serial_string, Base36};

    use super::*;

    #[test]
    fn test_from_base36() {
        let b36_string = "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833";
        let serial_string = serial_string::SerialString::new(
            crate::serialise::SerialiseType::Base36,
            b36_string.to_string(),
        );
        let key = Key::from(serial_string);

        let key: serial_string::SerialString = Base36::from(&key).into();
        crate::debug!("key: {key}");

        assert_eq!(
            key.get_string(),
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833"
        );
    }
}
