use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Unaligned)]
pub struct Signature {
    pub signature: [u8; 64],
}

impl Default for Signature {
    fn default() -> Self {
        Self { signature: [0u8; 64] }
    }
}

mod tests {
    use super::*;
    use zerocopy::AsBytes;
    use crate::crypto::sign;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};

    #[test]
    fn test_signature() {
        let private_key_bytes = base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let data = b"test";
        let signature_bytes = sign(data, &private_key_bytes);
        let signature: [u8; 64] = signature_bytes
            .as_slice()
            .try_into()
            .expect("ed25519 signature must be 64 bytes");

        let signature = Signature { signature };
        
        let signature_bytes = signature.as_bytes();
        
        println!("signature_bytes: {}", bytes_to_base36(&signature_bytes));
    }

}

