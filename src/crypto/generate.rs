use crate::model::address::Address;
use crate::model::key::Key;
use crate::model::private_address::PrivateAddress;
use ed25519_dalek::SigningKey;
use rand_core::OsRng;

impl PrivateAddress {
    pub fn generate() -> Self {
        let (private_key, public_key) = Self::generate_key();
        let address = Address::new(public_key);
        PrivateAddress::new(private_key, address)
    }
    fn generate_key() -> (Key, Key) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let private_key = Key::new(signing_key.to_bytes());
        let public_key = Key::new(verifying_key.to_bytes());

        (private_key, public_key)
    }
}
