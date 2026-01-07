
use crate::model::address::Address;

pub struct PrivateAddress {
    private_key: Vec<u8>,
    address: Address,
}
