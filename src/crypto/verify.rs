use ed25519_dalek::VerifyingKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Verifier;

pub fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let public_key: &[u8; 32] = public_key
        .try_into()
        .expect("ed25519 public key must be 32 bytes");
    let signature: &[u8; 64] = signature
        .try_into()
        .expect("ed25519 signature must be 64 bytes");
    let verifying_key = VerifyingKey::from_bytes(public_key).expect("ed25519 public key must be valid");
    let signature = Signature::from_bytes(signature);
    verifying_key.verify(data, &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::bytes_to_base36;
    use crate::crypto::generate_key;
    use crate::crypto::sign;

    #[test]
    fn test_verify() {
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

        assert!(verify(data, &signature, &public_key));
    }
}