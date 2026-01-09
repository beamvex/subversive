use crate::model::address::Address;
use crate::model::address::AddressTrait;
use crate::model::private_address::PrivateAddress;
use crate::model::private_address::PrivateAddressTrait;
use ed25519_dalek::SigningKey;
use rand_core::OsRng;

pub trait GenerateKey {
    fn generate_key() -> PrivateAddress {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let private_key = signing_key.to_bytes();
        let public_key = verifying_key.to_bytes();

        PrivateAddress::new(private_key, Address::new(public_key))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::generate::GenerateKey;
    use crate::model::address::Address;
    use crate::model::address::AddressTrait;
    use crate::model::private_address::PrivateAddress;
    use crate::model::private_address::PrivateAddressTrait;
    use crate::utils::bytes_to_base36;

    #[test]
    fn test_generate_key() {
        let private_key = PrivateAddress::generate_key();

        println!(
            "1. private_key_b36: {}",
            bytes_to_base36(private_key.get_private_key())
        );
        println!(
            "2. public_key_b36: {}",
            bytes_to_base36(private_key.get_address().get_public_key())
        );

        assert_eq!(private_key.get_address().get_public_key().len(), 32);
        assert_eq!(private_key.get_private_key().len(), 32);
    }
}
