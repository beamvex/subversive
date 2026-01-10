use crate::types::Key;
use ed25519_dalek::SigningKey;
use rand_core::OsRng;

pub fn generate_key() -> (Key, Key) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let private_key = signing_key.to_bytes();
    let public_key = verifying_key.to_bytes();

    (private_key, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::bytes_to_base36;

    #[test]
    fn test_generate_key() {
        let (private_key, public_key) = generate_key();

        println!("1. private_key_b36: {}", bytes_to_base36(&private_key));
        println!("2. public_key_b36: {}", bytes_to_base36(&public_key));

        assert_eq!(private_key.len(), 32);
        assert_eq!(public_key.len(), 32);
    }
}
