use crate::model::{Base36, FromBase36};
use sha3::{Digest, Keccak256};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Hash {
    bytes: [u8; 32],
}

impl FromBase36 for Hash {
    fn from_bytes(bytes: &[u8]) -> Self {
        Hash::read_from(bytes).unwrap()
    }
}

impl Hash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak256::default();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.into();
        Hash { bytes }
    }
}

impl From<Hash> for Base36 {
    fn from(hash: Hash) -> Self {
        Base36::from_bytes(&hash.bytes)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::transaction_data::TransactionData;
    use crate::model::PrivateAddress;

    #[test]
    fn test_hash() {
        let from_private_address = PrivateAddress::default();
        let from_address = from_private_address.get_address();
        let to_private_address = PrivateAddress::default();
        let to_address = to_private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let hash = Hash::from_bytes(&bytes);

        let hash: Base36 = hash.into();
        println!("hash: {}", hash.get_string());
    }
}
