use crate::model::Base36;
use crate::utils::{to_base36, FromBase36};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Key {
    bytes: [u8; 32],
}

impl Key {
    pub fn get_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl FromBase36 for Key {
    fn from_bytes(bytes: &[u8]) -> Self {
        Key::read_from(bytes).unwrap()
    }
}

impl From<[u8; 32]> for Key {
    fn from(bytes: [u8; 32]) -> Self {
        Key { bytes }
    }
}

impl From<Key> for Base36 {
    fn from(key: Key) -> Self {
        Base36::new(to_base36(&key.to_bytes()))
    }
}

#[cfg(test)]
mod tests {

    use crate::model::Key;
    use crate::utils::FromBase36;
    use crate::utils::ToBase36;

    #[test]
    fn test_from_base36() {
        let key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let key = key.to_base36();
        println!("key: {}", key);

        assert_eq!(key, "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");
    }
}
