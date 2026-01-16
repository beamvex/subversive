use crate::model::key::Key;
use crate::model::Signature;
use crate::utils::ToBase36;
use ed25519_dalek::{Signature as Ed25519Signature, Verifier, VerifyingKey};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Default, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Address {
    public_key: Key,
}

impl ToBase36 for Address {}

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

#[cfg(test)]
mod tests {

    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction::Transaction;

    use crate::utils::{FromBase36, ToBase36};

    #[test]
    fn test_verify() {
        let private_address = PrivateAddress::new();
        let from_address = Address::new(Key::from_base36(
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833",
        ));
        let to_address = Address::new(Key::from_base36(
            "1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso",
        ));

        let transaction = Transaction {
            from: from_address,
            to: to_address,
            amount: 1,
            timestamp: 0,
        };

        let bytes: Vec<u8> = {
            let borrowed_transaction = &transaction;
            borrowed_transaction.into()
        };
        let signature = private_address.sign(&bytes);

        let from_address = private_address.get_address();
        let verified = from_address.verify(&bytes, &signature);

        assert!(verified);
    }

    #[test]
    fn test_address() {
        let public_key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address::new(public_key);

        let address_bytes = address.to_base36();

        println!("address_bytes: {}", address_bytes);
    }
}
