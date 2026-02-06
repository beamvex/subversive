#[cfg(test)]
mod tests {

    use rand::rngs::OsRng;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs1v15::Pkcs1v15Sign;
    use rsa::pkcs8::LineEnding;
    use rsa::traits::PublicKeyParts;
    use rsa::{RsaPrivateKey, RsaPublicKey};
    use sha2::{Digest, Sha256};

    use crate::serialise::{Base36, SerialString};

    #[test]
    fn test_rsa() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 512).expect("failed to generate RSA key");
        assert_eq!(private_key.n().bits(), 512);

        let pem = private_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        let pem = pem.as_str();

        crate::debug!("pem {pem}");

        let _n = private_key.n();
        let _e = private_key.e();

        let test = b"test";
        let digest = Sha256::digest(test);
        let signresult = private_key.sign(Pkcs1v15Sign::new_unprefixed(), digest.as_slice());

        match signresult {
            Ok(signature) => {
                let str: SerialString = Base36::try_from(&signature).unwrap().into();
                crate::debug!("signature {str}");
            }
            Err(e) => {
                crate::debug!("signing failed: {e}");
            }
        }

        let public_key = private_key.to_public_key();
        let n = public_key.n().to_bytes_be();
        let str: SerialString = Base36::try_from(&n).unwrap().into();
        crate::debug!("n {str}");
    }
}
