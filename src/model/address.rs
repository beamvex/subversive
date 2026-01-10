use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

use crate::types::Key;

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Address {
    public_key: Key,
}

impl Address {
    pub fn new(public_key: Key) -> Self {
        Address { public_key }
    }
    pub fn get_public_key(&self) -> &Key {
        &self.public_key
    }
}

mod tests {
    use super::*;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};
    use zerocopy::AsBytes;

    #[test]
    fn test_address() {
        let private_key_bytes =
            base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let public_key: Key = private_key_bytes
            .as_slice()
            .try_into()
            .expect("base36_to_bytes_32 must return 32 bytes");

        let address = Address::new(public_key);

        let address_bytes = address.as_bytes();

        println!("address_bytes: {}", bytes_to_base36(&address_bytes));
    }
}
