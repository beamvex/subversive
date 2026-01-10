use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Key {
    bytes: [u8; 32],
}

impl Key {
    pub fn new(bytes: [u8; 32]) -> Self {
        Key { bytes }
    }

    pub fn get_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

mod tests {
    use crate::model::Key;
    use crate::utils::bytes_to_base36;

    #[test]
    fn test_from_base36() {
        let key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let key = bytes_to_base36(key.get_bytes());
        println!("key: {}", key);

        assert_eq!(key, "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");
    }
}
