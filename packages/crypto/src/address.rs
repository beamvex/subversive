// Moved from src/crypto/address.rs
use base58::ToBase58;

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub struct Address {
    private_key: SigningKey,
    public_key: VerifyingKey,
    public_address: String,
}

impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}

impl Address {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let public_address = Self::generate_address(&verifying_key);
        Self {
            private_key: signing_key,
            public_key: verifying_key,
            public_address,
        }
    }

    pub fn from_private_key(private_key: SigningKey) -> Self {
        let verifying_key = private_key.verifying_key();
        let public_address = Self::generate_address(&verifying_key);
        Self {
            private_key,
            public_key: verifying_key,
            public_address,
        }
    }

    fn generate_address(public_key: &VerifyingKey) -> String {
        let pub_key_bytes = public_key.to_bytes();
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(pub_key_bytes);
        let sha256_result = sha256_hasher.finalize();
        let mut ripemd160_hasher = Ripemd160::new();
        ripemd160_hasher.update(sha256_result);
        let ripemd160_result = ripemd160_hasher.finalize();
        ripemd160_result.to_base58()
    }

    pub fn get_private_key(&self) -> &SigningKey {
        &self.private_key
    }

    pub fn get_public_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    pub fn get_public_address(&self) -> &str {
        &self.public_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base58::FromBase58;
    use subversive_utils::test_utils::init_test_tracing;
    use tracing::info;

    #[test]
    fn test_address_generation() {
        init_test_tracing();
        let address = Address::new();
        info!(
            "Address: {} {} {}",
            address.get_public_address(),
            address.get_private_key().to_bytes().to_base58(),
            address.get_public_key().to_bytes().to_base58()
        );
        assert!(!address.get_private_key().to_bytes().is_empty());
        assert!(!address.get_public_key().to_bytes().is_empty());
        let public_address = address.get_public_address();
        assert!(!public_address.is_empty());
        assert!(public_address.from_base58().is_ok());
    }

    #[test]
    fn test_address_from_private_key() {
        init_test_tracing();
        let original = Address::new();
        let private_key = original.get_private_key().clone();
        let restored = Address::from_private_key(private_key);
        assert_eq!(original.get_public_address(), restored.get_public_address());
        assert_eq!(
            original.get_public_key().to_bytes(),
            restored.get_public_key().to_bytes()
        );
    }
}
