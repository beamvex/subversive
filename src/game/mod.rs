use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use simple_sign::{Signature, SignatureError, Signer};
use slahasher::{HashAlgorithm, Hashable};
use std::time::{Duration, SystemTime};

/// block in a chain
#[derive(Debug)]
pub struct Block {
    duration: Duration,
}

impl Block {
    /// Sign the block with the given signer
    ///
    /// # Errors
    ///
    /// Returns an error if the block cannot be hashed or the signer fails to sign
    pub fn try_sign(&self, signer: &impl Signer) -> Result<Signature, SignatureError> {
        match self.try_hash(HashAlgorithm::KECCAK512) {
            Ok(hash) => signer.sign(&hash),
            Err(e) => Err(SignatureError::new(e.to_string())),
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        let current_time = SystemTime::now();

        let duration = current_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| unreachable!());

        Self { duration }
    }
}

impl TryFrom<&Block> for ByteVec {
    type Error = SerialiseError;
    fn try_from(value: &Block) -> Result<Self, SerialiseError> {
        let fruity = false;
        if fruity {
            return Err(SerialiseError::new("feeling fruty".to_string()));
        }
        let duration = value.duration.as_millis().to_be_bytes().to_vec();
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&duration);

        Ok(Self::new(bytes))
    }
}

impl Hashable for Block {}
impl Encodable for Block {}

mod tests {
    use super::*;
    use simple_sign::Ed25519Signer;
    use slogger::debug;

    #[test]
    fn test_play() {
        let private_key = Ed25519Signer::new_random();

        //let bytes = base_xx::ByteVec::new();

        let block = Block::default();

        let signature = block.try_sign(&private_key);
        let signature = match &signature {
            Ok(signature) => signature,
            Err(e) => {
                eprintln!("Error: {e}");
                &Signature::default()
            }
        };

        debug!("signature {signature:?}");
        debug!("block {block:?}");

        let private_key2 = Ed25519Signer::new_random();

        let signature2 = block.try_sign(&private_key2);
        let signature2 = match &signature2 {
            Ok(signature) => signature,
            Err(e) => {
                eprintln!("Error: {e}");
                &Signature::default()
            }
        };

        debug!("signature {signature2:?}");

        let winner = signature.gt(&signature2);
        debug!("winner {winner:?}");
    }
}
