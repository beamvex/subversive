use crate::address::key::Key;
use crate::model::Base36;
use crate::model::Signature;

use ed25519_dalek::{Signature as Ed25519Signature, Verifier, VerifyingKey};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Address {
    public_key: Key,
}

impl Address {
    pub fn new(public_key: Key) -> Self {
        Address { public_key }
    }
    pub fn get_public_key(&self) -> &Key {
        &self.public_key
    }
    pub fn verify(&self, data: &[u8], signature: &Signature) -> bool {
        let public_key: &[u8] = self.get_public_key().as_bytes();
        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .expect("ed25519 public key must be valid"),
        )
        .unwrap();
        let signature = Ed25519Signature::from_bytes(signature.as_bytes().try_into().unwrap());
        verifying_key.verify(data, &signature).is_ok()
    }
}

impl From<&Address> for Address {
    fn from(value: &Address) -> Self {
        let public_key = value.get_public_key();
        let mut destination = [0u8; 32];
        destination.copy_from_slice(public_key.as_bytes());
        Address::new(Key::from(destination))
    }
}

impl From<&Address> for Base36 {
    fn from(address: &Address) -> Self {
        let public_key = address.get_public_key();
        let public_key: Vec<u8> = public_key.as_bytes().to_vec();
        Base36::from_bytes(&public_key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;

    use crate::model::base36::FromBase36;
    #[test]
    fn test_verify() {
        let private_address = PrivateAddress::default();
        let from_address = private_address.get_address();
        let to_address = private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        let from_address = private_address.get_address();
        let verified = from_address.verify(&bytes, &signature);

        assert!(verified);
    }

    #[test]
    fn test_address() {
        let public_key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address::new(public_key);

        let address_bytes: Base36 = (&address).into();

        println!("address_bytes: {}", address_bytes);
    }
}
