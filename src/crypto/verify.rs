use ed25519_dalek::Signature;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
use zerocopy::AsBytes;

use crate::model::Address;

impl Address {
    pub fn verify(&self, data: &[u8], signature: &crate::model::Signature) -> bool {
        let public_key: &[u8] = self.get_public_key().as_bytes();
        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .expect("ed25519 public key must be valid"),
        )
        .unwrap();
        let signature = Signature::from_bytes(signature.as_bytes().try_into().unwrap());
        verifying_key.verify(data, &signature).is_ok()
    }
}

#[cfg(test)]
mod tests {

    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction::Transaction;
    use crate::utils::FromBase36;

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
}
