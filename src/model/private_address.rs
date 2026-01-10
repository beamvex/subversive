use crate::model::address::Address;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: [u8; 32],
    address: Address,
}

impl PrivateAddress {
    pub fn new(private_key: [u8; 32], address: Address) -> Self {
        PrivateAddress {
            private_key,
            address,
        }
    }
    pub fn get_private_key(&self) -> &[u8; 32] {
        &self.private_key
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

        let public_key: Key = public_key_bytes
            .as_slice()
            .try_into()
            .expect("base36_to_bytes_32 must return 32 bytes");

        let private_key: Key = private_key_bytes
            .as_slice()
            .try_into()
            .expect("base36_to_bytes_32 must return 32 bytes");

        let address = Address::new(public_key);

        let private_address = PrivateAddress::new(private_key, address);

        let private_address_bytes = private_address.as_bytes();

        println!(
            "private_address_bytes: {}",
            bytes_to_base36(&private_address_bytes)
        );
    }
}
