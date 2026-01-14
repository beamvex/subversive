use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;

use crate::model::private_address::PrivateAddress;
use crate::model::transaction::Transaction;
use crate::model::Signature;

pub trait Sign {
    fn sign(&self, private_address: &PrivateAddress) -> Signature
    where
        Self: zerocopy::AsBytes,
    {
        let bytes: &[u8] = self.as_bytes();

        let signing_key = SigningKey::from_bytes(private_address.get_private_key().get_bytes());
        let signature = signing_key.sign(bytes);
        Signature::new(signature.to_bytes())
    }
}

impl Sign for Transaction {}

#[cfg(test)]
mod tests {

    use crate::crypto::sign::Sign;
    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction::Transaction;
    use crate::model::Signature;
    use crate::utils::{FromBase36, ToBase36};

    #[test]
    fn test_sign() {
        let private_address = PrivateAddress::generate();
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

        let signature = transaction.sign(&private_address);

        println!("signature: {}", signature.to_base36());
    }
}
