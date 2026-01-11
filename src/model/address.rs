use crate::model::key::Key;
use crate::utils::ToBase36;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Address {
    public_key: Key,
}

impl ToBase36 for Address {}

impl Address {
    pub fn new(public_key: Key) -> Self {
        Address { public_key }
    }
    pub fn get_public_key(&self) -> &Key {
        &self.public_key
    }
}

mod tests {

    use crate::model::key::Key;
    use crate::model::Address;
    use crate::utils::{FromBase36, ToBase36};

    #[test]
    fn test_address() {
        let public_key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address::new(public_key);

        let address_bytes = address.to_base36();

        println!("address_bytes: {}", address_bytes);
    }
}
