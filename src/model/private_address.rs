use crate::model::{address::Address, Key};
use crate::utils::ToBase36;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: Key,
    address: Address,
}

impl ToBase36 for PrivateAddress {}

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
    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::utils::{FromBase36, ToBase36};
    use zerocopy::AsBytes;

    #[test]
    fn test_private_address() {
        let public_key: Key =
            Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let private_key: Key =
            Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address::new(public_key);

        let private_address = PrivateAddress::new(private_key, address);

        let private_address_bytes = private_address.to_base36();

        println!("private_address_bytes: {}", private_address_bytes);
    }

    #[test]
    fn test_generate_key() {
        let private_address = PrivateAddress::generate();

        println!(
            "1. private_key_b36: {}",
            private_address.get_private_key().to_base36()
        );
        println!(
            "2. public_key_b36: {}",
            private_address.get_address().get_public_key().to_base36()
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

        let private_address_bytes = private_address.to_base36();

        println!("private_address_bytes: {}", private_address_bytes);
    }
}
