use crate::utils::{FromBase36, ToBase36};
use sha3::{Digest, Keccak256};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Hash {
    bytes: [u8; 32],
}

impl ToBase36 for Hash {}

impl FromBase36 for Hash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Hash::read_from(bytes).unwrap()
    }
}

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Hash { bytes }
    }
}

impl Hash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak256::default();
        hasher.update(bytes);
        let result = hasher.finalize();
        Hash::new(result.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::transaction_data::TransactionData;
    use crate::model::PrivateAddress;
    use crate::utils::ToBase36;

    #[test]
    fn test_hash() {
        let from_private_address = PrivateAddress::new();
        let from_address = from_private_address.get_address();
        let to_private_address = PrivateAddress::new();
        let to_address = to_private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let hash = Hash::from_bytes(&bytes);

        println!("hash: {}", hash.to_base36());
    }
}
