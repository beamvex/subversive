use crate::model::{address::Address, Key};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: Key,
    address: Address,
}

impl PrivateAddress {
    pub fn new(private_key: Key, address: Address) -> Self {
        PrivateAddress {
            private_key,
            address,
        }
    }

    pub fn get_private_key(&self) -> &Key {
        &self.private_key
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }
}

mod tests {
    use super::*;
    use crate::types::Key;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};
    use zerocopy::AsBytes;

    #[test]
    fn test_private_address() {
        let private_key_bytes =
            base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let public_key_bytes =
            base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let public_key: Key = Key::from_bytes(public_key_bytes);

        let private_key: Key = Key::from_bytes(private_key_bytes);

        let address = Address::new(public_key);

        let private_address = PrivateAddress::new(private_key, address);

        let private_address_bytes = private_address.as_bytes();

        println!(
            "private_address_bytes: {}",
            bytes_to_base36(&private_address_bytes)
        );
    }

    #[test]
    fn test_generate_key() {
        let private_address = PrivateAddress::generate();

        println!(
            "1. private_key_b36: {}",
            bytes_to_base36(private_address.get_private_key().get_bytes())
        );
        println!(
            "2. public_key_b36: {}",
            bytes_to_base36(private_address.get_address().get_public_key().get_bytes())
        );

        assert_eq!(private_address.get_private_key().get_bytes().len(), 32);
        assert_eq!(
            private_address
                .get_address()
                .get_public_key()
                .get_bytes()
                .len(),
            32
        );

        let private_address_bytes = private_address.as_bytes();

        println!(
            "private_address_bytes: {}",
            bytes_to_base36(&private_address_bytes)
        );
    }
}
