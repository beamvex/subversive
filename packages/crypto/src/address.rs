use std::str::FromStr;

// Moved from src/crypto/address.rs
use base58::{FromBase58, ToBase58};

use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub struct Address {
    private_key: Option<SigningKey>,
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
            private_key: Some(signing_key),
            public_key: verifying_key,
            public_address,
        }
    }

    pub fn from_private_key(private_key: SigningKey) -> Self {
        let verifying_key = private_key.verifying_key();
        let public_address = Self::generate_address(&verifying_key);
        Self {
            private_key: Some(private_key),
            public_key: verifying_key,
            public_address,
        }
    }

    pub fn from_public_address(public_address: &str) -> Result<Self, &'static str> {
        // Decode base58 public address
        let decoded = public_address
            .from_base58()
            .map_err(|_| "Invalid base58 address")?;

        // Convert to VerifyingKey
        let public_key =
            VerifyingKey::from_bytes(&decoded).map_err(|_| "Invalid public key bytes")?;

        // Verify the address matches what we would generate
        let generated_address = Self::generate_address(&public_key);
        if generated_address != public_address {
            return Err("Address checksum mismatch");
        }

        Ok(Self {
            private_key: None,
            public_key,
            public_address: public_address.to_string(),
        })
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

    pub fn get_private_key(&self) -> Option<&SigningKey> {
        self.private_key.as_ref()
    }

    pub fn get_public_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    pub fn get_public_address(&self) -> &str {
        &self.public_address
    }

    pub fn sign(&mut self, message: &str) -> Result<String, &'static str> {
        if let Some(key) = &self.private_key {
            let signature = key.sign(message.as_bytes());
            Ok(signature.to_string())
        } else {
            Err("Cannot sign: address is public-only")
        }
    }

    pub fn verify(&self, message: &str, signature: &str) -> bool {
        if let Ok(sig) = ed25519_dalek::Signature::from_str(signature) {
            self.public_key
                .verify_strict(message.as_bytes(), &sig)
                .is_ok()
        } else {
            false
        }
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
            address.get_private_key().unwrap().to_bytes().to_base58(),
            address.get_public_key().to_bytes().to_base58()
        );
        assert!(!address.get_private_key().unwrap().to_bytes().is_empty());
        assert!(!address.get_public_key().to_bytes().is_empty());
        let public_address = address.get_public_address();
        assert!(!public_address.is_empty());
        assert!(public_address.from_base58().is_ok());
    }

    #[test]
    fn test_address_from_private_key() {
        init_test_tracing();
        let original = Address::new();
        let private_key = original.get_private_key().unwrap().clone();
        let restored = Address::from_private_key(private_key);
        assert_eq!(original.get_public_address(), restored.get_public_address());
        assert_eq!(
            original.get_public_key().to_bytes(),
            restored.get_public_key().to_bytes()
        );
    }

    #[test]
    fn test_address_from_public_address() {
        init_test_tracing();
        let original = Address::new();
        let public_address = original.get_public_address();
        let restored = Address::from_public_address(public_address).unwrap();
        assert_eq!(original.get_public_address(), restored.get_public_address());
        assert_eq!(
            original.get_public_key().to_bytes(),
            restored.get_public_key().to_bytes()
        );
    }

    #[test]
    fn test_message_signing() {
        init_test_tracing();
        let mut address = Address::new();
        let message = "Hello, World!";
        info!("Message: {}", message);
        let signature = address.sign(message).unwrap();
        info!("Signature: {}", signature);

        // Verify the signature
        assert!(address.verify(message, &signature));

        // Verify that invalid signatures are rejected
        assert!(!address.verify(message, "invalid-signature"));
        assert!(!address.verify("different-message", &signature));
    }

    #[test]
    fn test_public_only_address() {
        init_test_tracing();
        let mut original = Address::new();
        let pub_addr = original.get_public_address();

        // Create address from public address
        let mut pub_only = Address::from_public_address(pub_addr).unwrap();
        assert!(pub_only.get_private_key().is_none());
        assert!(pub_only.sign("message").is_err());

        // Should still be able to verify signatures
        let message = "test message";
        let signature = original.sign(message).unwrap();
        assert!(pub_only.verify(message, &signature));

        // Test invalid public addresses
        assert!(Address::from_public_address("invalid-address").is_err());
        assert!(Address::from_public_address("").is_err());
    }
}
