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

mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let signature = Signature::from_base36("1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso");

        println!("signature: {}", signature.to_base36());
    }
}
