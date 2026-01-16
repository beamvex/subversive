use crate::model::{address::Address, Key, Signature};
use crate::utils::ToBase36;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;

use rand_core::OsRng;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct PrivateAddress {
    private_key: Key,
    address: Address,
}

impl ToBase36 for PrivateAddress {}

impl PrivateAddress {
    pub fn new() -> Self {
        let (private_key, public_key) = Self::generate_key();
        let address = Address::new(public_key);

        PrivateAddress {
            private_key,
            address,
        }
    }

    pub fn get_private_key(&self) -> &Key {
        &self.private_key
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }

    fn generate_key() -> (Key, Key) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let private_key = Key::new(signing_key.to_bytes());
        let public_key = Key::new(verifying_key.to_bytes());

        (private_key, public_key)
    }
}

impl PrivateAddress {
    pub fn sign(&self, bytes: &[u8]) -> Signature {
        let signing_key = SigningKey::from_bytes(self.get_private_key().get_bytes());
        let signature = signing_key.sign(bytes);
        Signature::new(signature.to_bytes())
    }
}

#[cfg(test)]
mod tests {

    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;
    use crate::utils::ToBase36;

    #[test]
    fn test_generate_key() {
        let private_address = PrivateAddress::new();

        println!(
            "1. private_key_b36: {}",
            private_address.get_private_key().to_base36()
        );
        println!(
            "2. public_key_b36: {}",
            private_address.get_address().get_public_key().to_base36()
        );

        assert_eq!(private_address.get_private_key().get_bytes().len(), 32);
        assert_eq!(
            private_address
                .get_address()
                .get_public_key()
                .get_bytes()
                .len(),
            32
        );

        let private_address_bytes = private_address.to_base36();

        println!("private_address_bytes: {}", private_address_bytes);
    }

    #[test]
    fn test_sign() {
        let from_private_address = PrivateAddress::new();
        let from_address = from_private_address.get_address();
        let to_private_address = PrivateAddress::new();
        let to_address = to_private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        println!("signature: {}", signature.to_base36());
    }
}
