#[cfg(test)]
mod tests {

    use rand::rngs::OsRng;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs8::LineEnding;
    use rsa::traits::PublicKeyParts;
    use rsa::RsaPrivateKey;

    use crate::address::private_address;

    #[test]
    fn test_rsa() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate RSA key");
        assert_eq!(private_key.n().bits(), 1024);
        let pem = private_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        println!("prem {}", pem.as_str());

        let test = b"test";
        let signature = private_key.sign(test);
        println!("signature {}", signature.as_str());
    }
}
