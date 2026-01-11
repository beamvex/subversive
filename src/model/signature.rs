use crate::utils::{FromBase36, ToBase36};
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

impl ToBase36 for Signature {}

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
        let b36 = signature.to_base36();
        println!("signature_b36: {}", b36);

        let signature2 = Signature::from_base36(&b36);
        assert_eq!(signature.get_signature(), signature2.get_signature());
    }
}
