
use crate::model::address::Address;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    pub private_key: [u8; 32],
    pub address: Address,
}
