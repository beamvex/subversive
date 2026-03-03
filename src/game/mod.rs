mod tests {
    use simple_sign::{Ed25519Signer, Signer};
    use slahasher::{Hash, HashAlgorithm};

    #[test]
    fn test_play() {
        let private_key = Ed25519Signer::new_random();

        let current_time = std::time::SystemTime::now();

        match current_time.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let bytes = base_xx::ByteVec::new(duration.as_secs().to_be_bytes().to_vec());
                match Hash::try_hash(&bytes, HashAlgorithm::KECCAK512) {
                    Ok(hash) => {
                        let signature = private_key.sign(&hash);
                        assert!(signature.is_ok());
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                assert!(false);
            }
        }
    }
}
