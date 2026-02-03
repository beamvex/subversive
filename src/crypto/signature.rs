use crate::{
    crypto::SigningAlgorithm,
    hashable,
    serialise::{AsBytes, FromBytes},
};

use crate::serialisable;

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
    pub fn new(signature: Vec<u8>) -> Self {
        Signature {
            algorithm: SigningAlgorithm::ED25519,
            signature,
        }
    }

    #[must_use]
    pub fn new_with_algorithm(algorithm: SigningAlgorithm, signature: Vec<u8>) -> Self {
        Signature {
            algorithm,
            signature,
        }
    }

    #[must_use]
    pub fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }

    #[must_use]
    pub fn get_algorithm(&self) -> SigningAlgorithm {
        self.algorithm
    }
}

impl From<&Signature> for Vec<u8> {
    fn from(value: &Signature) -> Self {
        value.as_bytes().clone()
    }
}

impl AsBytes for Signature {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.algorithm.into());
        bytes.extend_from_slice(&self.signature);
        bytes
    }
}

impl FromBytes for Signature {
    fn from_bytes(bytes: &[u8]) -> Self {
        let algorithm = SigningAlgorithm::from(bytes[0]);
        let bytes = bytes[1..].to_vec();
        Signature::new_with_algorithm(algorithm, bytes)
    }
}

serialisable!(Signature);

hashable!(Signature);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::Hash;
    use crate::hashing::Keccak256;
    use crate::hashing::Keccak384;
    use crate::hashing::Sha256;

    use crate::serialise::Base36;
    use crate::serialise::SerialString;

    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;

    #[test]
    fn test_signature() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let data = b"test";
        let sig = signing_key.sign(data).to_bytes().to_vec();

        let signature = Signature::new_with_algorithm(SigningAlgorithm::ED25519, sig);

        let serialised: SerialString = Base36::from(&signature).into();

        crate::debug!("serialised: {}", serialised);
        crate::debug!("serialised debug: {:?}", serialised);

        let signature2: Signature = serialised.into();
        crate::debug!("signature2: {:?}", signature2.get_signature());

        assert_eq!(signature.get_signature(), signature2.get_signature());

        let hash: Hash = Keccak256::from(&signature).into();
        let hash_str: SerialString = Base36::from(&hash).into();
        crate::debug!("signature hash keccak-256: {}", hash_str);

        let hash: Hash = Sha256::from(&signature).into();
        let hash_str: SerialString = Base36::from(&hash).into();
        crate::debug!("signature hash sha2-256: {}", hash_str);

        let hash: Hash = Keccak384::from(&signature).into();
        let hash_str: SerialString = Base36::from(&hash).into();
        crate::debug!("signature hash keccak-384: {}", hash_str);
    }
}
