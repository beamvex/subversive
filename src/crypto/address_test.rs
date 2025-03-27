#[cfg(test)]
mod tests {
    use crate::crypto::address::Address;

    use base58::FromBase58;

    #[test]
    fn test_address_generation() {
        let address = Address::new();

        // Check that private key exists
        assert!(!address.get_private_key().as_ref().is_empty());

        // Check that public key exists
        assert!(!address.get_public_key().serialize().is_empty());

        // Check that public address is valid base58
        let public_address = address.get_public_address();
        assert!(!public_address.is_empty());
        assert!(public_address.from_base58().is_ok());
    }

    #[test]
    fn test_address_from_private_key() {
        let original = Address::new();
        let private_key = *original.get_private_key();

        let restored = Address::from_private_key(private_key);

        assert_eq!(original.get_public_address(), restored.get_public_address());
        assert_eq!(
            original.get_public_key().serialize(),
            restored.get_public_key().serialize()
        );
    }
}
