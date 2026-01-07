use zerocopy::{Immutable, IntoBytes};

#[repr(C)]
#[derive(Debug, Default, Immutable, IntoBytes)]
pub struct Address {
    public_key: Vec<u8>,
}

mod tests {
    use super::*;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};

    #[test]
    fn test_address() {
        let private_key_bytes = base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address{public_key: private_key_bytes.to_vec()};
        
        let address_bytes = address.as_bytes();
        
        println!("address_bytes: {}", bytes_to_base36(&address_bytes));
    }

}
