pub use crate::crypto::generate::GenerateKey;
use crate::model::address::Address;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

pub trait PrivateAddressTrait {
    fn new(private_key: [u8; 32], address: Address) -> Self;
    fn get_private_key(&self) -> &[u8; 32];
    fn get_address(&self) -> &Address;
}

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: [u8; 32],
    address: Address,
}

impl GenerateKey for PrivateAddress {}

impl PrivateAddressTrait for PrivateAddress {
    fn new(private_key: [u8; 32], address: Address) -> Self {
        PrivateAddress {
            private_key,
            address,
        }
    }
    fn get_private_key(&self) -> &[u8; 32] {
        &self.private_key
    }
    fn get_address(&self) -> &Address {
        &self.address
    }
}
