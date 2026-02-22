use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};

use crate::crypto::SigningAlgorithm;

/// A cryptographic signature along with its associated signing algorithm.
pub struct Signature {
    algorithm: SigningAlgorithm,
    signature: Vec<u8>,
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            algorithm: SigningAlgorithm::ED25519,
            signature: vec![0u8; 64],
        }
    }
}

impl Signature {
    #[must_use]
    /// Creates a new Ed25519 signature wrapper from raw signature bytes.
    pub const fn new(signature: Vec<u8>) -> Self {
        Self {
            algorithm: SigningAlgorithm::ED25519,
            signature,
        }
    }

    #[must_use]
    /// Creates a new signature wrapper from raw signature bytes and an explicit algorithm.
    pub const fn new_with_algorithm(algorithm: SigningAlgorithm, signature: Vec<u8>) -> Self {
        Self {
            algorithm,
            signature,
        }
    }

    #[must_use]
    /// Returns the raw signature bytes.
    pub const fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }

    #[must_use]
    /// Returns the algorithm used for this signature.
    pub const fn get_algorithm(&self) -> SigningAlgorithm {
        self.algorithm
    }
}

impl TryFrom<&Signature> for ByteVec {
    type Error = SerialiseError;
    fn try_from(value: &Signature) -> Result<Self, Self::Error> {
        let algorithm: u8 = value.get_algorithm().into();
        let mut bytes = vec![algorithm];
        bytes.extend_from_slice(value.get_signature());
        Ok(Self::new(bytes))
    }
}

impl TryFrom<ByteVec> for Signature {
    type Error = SerialiseError;
    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        match SigningAlgorithm::try_from(value.get_bytes()[0]) {
            Ok(algorithm) => {
                let bytes = value.get_bytes()[1..].to_vec();
                Ok(Self::new_with_algorithm(algorithm, bytes))
            }
            Err(e) => Err(e),
        }
    }
}

impl Encodable for Signature {}

#[cfg(test)]
mod tests {
    use super::*;

    use base_xx::EncodedString;
    use base_xx::Encoding;
    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;

    use slogger::debug;

    #[test]
    fn test_signature() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let data = b"test big long data test test big long data testtest big long data testtest big long data testtest big long data testtest big long data testtest big long data testtest big long data testtest big long data testtest big long data test";
        let sig = signing_key.sign(data).to_bytes().to_vec();

        let signature = Signature::new_with_algorithm(SigningAlgorithm::ED25519, sig);

        let serialised = signature
            .try_encode(Encoding::Base36)
            .unwrap_or_else(|_| EncodedString::new(Encoding::Base36, String::new()));

        debug!("serialised: {serialised}");
        debug!("serialised debug: {serialised:?}");
    }
}
