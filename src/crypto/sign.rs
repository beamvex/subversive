use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;

use crate::model::private_address::PrivateAddress;
use crate::model::Signature;

impl PrivateAddress {
    pub fn sign(&self, bytes: &[u8]) -> Signature {
        let signing_key = SigningKey::from_bytes(self.get_private_key().get_bytes());
        let signature = signing_key.sign(bytes);
        Signature::new(signature.to_bytes())
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
    fn test_sign() {
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

        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        println!("signature: {}", signature.to_base36());
    }
}
