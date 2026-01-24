use crate::{model::Base36, model::FromBase36};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Signature {
    signature: [u8; 64],
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            signature: [0u8; 64],
        }
    }
}

impl Signature {
    pub fn new(signature: [u8; 64]) -> Self {
        Signature { signature }
    }
    pub fn get_signature(&self) -> &[u8; 64] {
        &self.signature
    }
}

impl FromBase36 for Signature {
    fn from_bytes(bytes: &[u8]) -> Self {
        Signature::read_from(bytes).unwrap()
    }
}

impl From<&Signature> for Vec<u8> {
    fn from(value: &Signature) -> Self {
        value.as_bytes().to_vec()
    }
}

impl From<&Signature> for Base36 {
    fn from(signature: &Signature) -> Self {
        Base36::from_bytes(&signature.get_signature().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;
    use rand_core::OsRng;

    #[test]
    fn test_signature() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let data = b"test";
        let sig = signing_key.sign(data).to_bytes();

        let signature = Signature::new(sig);
        let b36: Base36 = (&signature).into();
        println!("signature_b36: {}", b36);

        let signature_b36: Base36 = (&signature).into();
        let signature2 = Signature::from_base36(&signature_b36.get_string());
        assert_eq!(signature.get_signature(), signature2.get_signature());
    }
}
