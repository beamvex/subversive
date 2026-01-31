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
    pub fn new(signature: Vec<u8>) -> Self {
        Signature {
            algorithm: SigningAlgorithm::ED25519,
            signature,
        }
    }

    pub fn new_with_algorithm(algorithm: SigningAlgorithm, signature: Vec<u8>) -> Self {
        Signature {
            algorithm,
            signature,
        }
    }

    pub fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }

    pub fn get_algorithm(&self) -> SigningAlgorithm {
        self.algorithm
    }
}

impl From<&Signature> for Vec<u8> {
    fn from(value: &Signature) -> Self {
        value.as_bytes().to_vec()
    }
}

impl AsBytes for Signature {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.algorithm as u8);
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
    use crate::serialise::SerialiseType;

    use crate::hashing::HashAlgorithm;
    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;

    #[test]
    fn test_signature() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let data = b"test";
        let sig = signing_key.sign(data).to_bytes().to_vec();

        let signature = Signature::new_with_algorithm(SigningAlgorithm::ED25519, sig);

        let serialised = signature.into_serial_string(SerialiseType::Base36);

        println!("serialised: {}", serialised);
        println!("serialised debug: {:?}", serialised);

        let signature2: Signature = (&serialised).into();
        /*
        assert_eq!(signature.get_signature(), signature2.get_signature());

        let hash = signature.hash(HashAlgorithm::KECCAK256);
        let hash_str = hash.into_serial_string(SerialiseType::Base36);
        println!("signature hash keccak-256: {}", hash_str);

        let hash = signature.hash(HashAlgorithm::SHA256);
        let hash_str = hash.into_serial_string(SerialiseType::Base36);
        println!("signature hash sha2-256: {}", hash_str);

        let hash = signature.hash(HashAlgorithm::KECCAK384);
        let hash_str = hash.into_serial_string(SerialiseType::Base36);
        println!("signature hash keccak-384: {}", hash_str);
        */
    }
}
