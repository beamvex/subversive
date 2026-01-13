use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;

use crate::model::private_address::PrivateAddress;
use crate::model::Signature;

pub trait Sign {
    fn sign(&self, private_address: &PrivateAddress) -> Signature {
        let bytes: &[u8] = b"data";

        let signing_key = SigningKey::from_bytes(private_address.get_private_key().get_bytes());
        let signature = signing_key.sign(bytes);
        Signature::new(signature.to_bytes())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::bytes_to_base36;
    use crate::crypto::generate_key;

    #[test]
    fn test_sign() {
        let (private_key, public_key) = generate_key();

        let data = b"test";
        let signature = sign(data, &private_key);

        println!("1. private_key_b36: {}", bytes_to_base36(&private_key));
        println!("2. public_key_b36: {}", bytes_to_base36(&public_key));
        println!("3. data: {}", bytes_to_base36(data));
        println!("4. signature: {}", bytes_to_base36(&signature));

        assert_eq!(private_key.len(), 32);
        assert_eq!(public_key.len(), 32);
        assert_eq!(data.len(), 4);
        assert_eq!(signature.len(), 64);
    }
}

    */
