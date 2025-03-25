use rand::rngs::OsRng;
use ripemd::{Digest, Ripemd160};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::Sha256;
use base58::ToBase58;

pub struct Address {
    private_key: SecretKey,
    public_key: PublicKey,
    public_address: String,
}

impl Address {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        
        // Generate private key
        let private_key = SecretKey::new(&mut rng);
        
        // Generate public key
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        
        // Generate public address
        let public_address = Self::generate_address(&public_key);
        
        Self {
            private_key,
            public_key,
            public_address,
        }
    }

    pub fn from_private_key(private_key: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let public_address = Self::generate_address(&public_key);
        
        Self {
            private_key,
            public_key,
            public_address,
        }
    }

    fn generate_address(public_key: &PublicKey) -> String {
        // Serialize public key
        let pub_key_serialized = public_key.serialize_uncompressed();
        
        // SHA256
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&pub_key_serialized);
        let sha256_result = sha256_hasher.finalize();
        
        // RIPEMD160
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha256_result);
        let ripemd_result = ripemd_hasher.finalize();
        
        // Base58 encode
        ripemd_result.to_base58()
    }

    pub fn get_private_key(&self) -> &SecretKey {
        &self.private_key
    }

    pub fn get_public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn get_public_address(&self) -> &str {
        &self.public_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let address = Address::new();
        
        // Check that private key exists
        assert!(address.get_private_key().as_ref().len() > 0);
        
        // Check that public key exists
        assert!(address.get_public_key().serialize().len() > 0);
        
        // Check that public address is valid base58
        let public_address = address.get_public_address();
        assert!(!public_address.is_empty());
        assert!(public_address.from_base58().is_ok());
    }

    #[test]
    fn test_address_from_private_key() {
        let original = Address::new();
        let private_key = original.get_private_key().clone();
        
        let restored = Address::from_private_key(private_key);
        
        assert_eq!(original.get_public_address(), restored.get_public_address());
        assert_eq!(
            original.get_public_key().serialize(),
            restored.get_public_key().serialize()
        );
    }
}
