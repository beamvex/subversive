use crate::{algorithm::AlgorithmType, hashable, serialise};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes)]
pub struct Signature {
    algorithm_type: AlgorithmType,
    signature: [u8; 64],
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            algorithm_type: AlgorithmType::ED25519,
            signature: [0u8; 64],
        }
    }
}

impl Signature {
    pub fn new(signature: [u8; 64]) -> Self {
        Signature {
            algorithm_type: AlgorithmType::ED25519,
            signature,
        }
    }

    pub fn new_with_algorithm(algorithm_type: AlgorithmType, signature: [u8; 64]) -> Self {
        Signature {
            algorithm_type,
            signature,
        }
    }

    pub fn get_signature(&self) -> &[u8; 64] {
        &self.signature
    }

    pub fn get_algorithm_type(&self) -> AlgorithmType {
        self.algorithm_type
    }
}

impl From<&Signature> for Vec<u8> {
    fn from(value: &Signature) -> Self {
        value.as_bytes().to_vec()
    }
}

serialise!(Signature);
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
        let sig = signing_key.sign(data).to_bytes();

        let signature = Signature::new_with_algorithm(AlgorithmType::ED25519, sig);

        let serialised = signature.into_serial_string(SerialiseType::Base36);

        println!("serialised: {}", serialised);
        println!("serialised debug: {:?}", serialised);

        let signature2: Signature = (&serialised).into();

        assert_eq!(signature.get_signature(), signature2.get_signature());

        let hash = signature.hash(HashAlgorithm::KECCAK256);
        println!("signature hash: {:?}", hash);

        let hash = signature.hash(HashAlgorithm::SHA256);
        println!("signature hash sha2-256: {:?}", hash);
    }
}
