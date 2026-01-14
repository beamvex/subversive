use ed25519_dalek::Signature;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;

pub fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let public_key: &[u8; 32] = public_key
        .try_into()
        .expect("ed25519 public key must be 32 bytes");
    let signature: &[u8; 64] = signature
        .try_into()
        .expect("ed25519 signature must be 64 bytes");
    let verifying_key =
        VerifyingKey::from_bytes(public_key).expect("ed25519 public key must be valid");
    let signature = Signature::from_bytes(signature);
    verifying_key.verify(data, &signature).is_ok()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::crypto::sign::Sign;
    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction::Transaction;
    use crate::model::Signature;
    use crate::utils::{FromBase36, ToBase36};
    use zerocopy::AsBytes;

    #[test]
    fn test_verify() {
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

        let data: &[u8] = transaction.as_bytes();

        let verified = verify(
            data,
            signature.as_bytes(),
            private_address.get_address().as_bytes(),
        );

        assert!(verified);
    }
}
