use base_xx::{byte_vec::Encodable, ByteVec};

/// A test struct for serialisation.
pub struct Test {
    data: Vec<u8>,
}

impl Test {
    /// Creates a new Test instance from raw data.
    #[must_use]
    pub const fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl TryFrom<ByteVec> for Test {
    type Error = base_xx::SerialiseError;

    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        Ok(Test {
            data: value.get_bytes().to_vec(),
        })
    }
}

impl TryFrom<&Test> for ByteVec {
    type Error = base_xx::SerialiseError;

    fn try_from(value: &Test) -> Result<Self, Self::Error> {
        Ok(ByteVec::new(value.data.clone()))
    }
}

impl Encodable for Test {}

#[cfg(test)]
mod tests {

    use super::*;
    use base_xx::Encoding;
    use rand::rngs::OsRng;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs1v15::Pkcs1v15Sign;
    use rsa::pkcs8::LineEnding;
    use rsa::traits::PublicKeyParts;
    use rsa::RsaPrivateKey;
    use sha2::{Digest, Sha256};

    use slogger::debug;

    #[test]
    fn test_rsa() {
        let mut rng = OsRng;
        match RsaPrivateKey::new(&mut rng, 256) {
            Ok(private_key) => {
                assert_eq!(private_key.n().bits(), 256);

                match private_key.to_pkcs1_pem(LineEnding::LF) {
                    Ok(pem) => {
                        let pem = pem.as_str();
                        debug!("pem {pem}");
                    }
                    Err(e) => {
                        debug!("failed to generate RSA key: {e}");
                    }
                }

                let _n = private_key.n();
                let _e = private_key.e();

                let test = b"test";
                let digest = Sha256::digest(test);
                let signresult =
                    private_key.sign(Pkcs1v15Sign::new_unprefixed(), digest.as_slice());

                match signresult {
                    Ok(signature) => {
                        let bytes = Test::new(signature);
                        match bytes.try_encode(Encoding::Base36) {
                            Ok(encoded) => {
                                debug!("signature {encoded}");
                            }
                            Err(e) => {
                                debug!("failed to encode signature: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        debug!("signing failed: {e}");
                    }
                }

                let public_key = private_key.to_public_key();
                let n = public_key.n().to_bytes_be();
                let bytes = Test::new(n);
                match bytes.try_encode(Encoding::Base36) {
                    Ok(encoded) => {
                        debug!("n {encoded}");
                    }
                    Err(e) => {
                        debug!("failed to encode n: {e}");
                    }
                }
            }
            Err(e) => {
                debug!("failed to generate RSA key: {e}");
            }
        }
    }
}
