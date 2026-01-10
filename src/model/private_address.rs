pub use crate::crypto::generate::GenerateKey;
use crate::model::address::Address;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: [u8; 32],
    address: Address,
}

impl PrivateAddress {
    fn get_private_key(&self) -> &[u8; 32] {
        &self.private_key
    }
    fn get_address(&self) -> &Address {
        &self.address
    }
}
