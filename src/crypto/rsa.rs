#[cfg(test)]
mod tests {

    use rand::rngs::OsRng;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs1v15::Pkcs1v15Sign;
    use rsa::pkcs8::LineEnding;
    use rsa::traits::PublicKeyParts;
    use rsa::RsaPrivateKey;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_rsa() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate RSA key");
        assert_eq!(private_key.n().bits(), 1024);
        let pem = private_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        crate::debug!("prem {}", pem.as_str());

        let test = b"test";
        let digest = Sha256::digest(test);
        let signature = private_key
            .sign(Pkcs1v15Sign::new_unprefixed(), digest.as_slice())
            .unwrap();
        crate::debug!("signature_len {}", signature.len());
        crate::debug!("signature {:?}", signature.as_slice());
    }
}
