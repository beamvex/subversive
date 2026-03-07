use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use chrono::{DateTime, Utc};
use simple_sign::{Signature, SignatureError, Signer};
use slahasher::{Hash, HashAlgorithm, Hashable};

/// block in a chain
#[derive(Debug)]
pub struct Block {
    time: DateTime<Utc>,
    version: u8,
    root_hash: Hash,
    previous_block_hash: Hash,
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
    /// Create the "Genesis" block :D
    fn default() -> Self {
        let time = DateTime::default();

        let time_millis = time.timestamp_millis();
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&time_millis.to_be_bytes());
        let bytes = ByteVec::new(bytes);

        let root_hash = Hash::try_hash(&bytes, HashAlgorithm::KECCAK512)
            .unwrap_or_else(|_| Hash::new(HashAlgorithm::KECCAK512, ByteVec::new(vec![])));
        let previous_block_hash = Hash::try_hash(root_hash.get_bytes(), HashAlgorithm::KECCAK512)
            .unwrap_or_else(|_| Hash::new(HashAlgorithm::KECCAK512, ByteVec::new(vec![])));

        Self {
            time,
            version: 1,
            root_hash,
            previous_block_hash,
        }
    }
}

impl TryFrom<&Block> for ByteVec {
    type Error = SerialiseError;
    fn try_from(value: &Block) -> Result<Self, SerialiseError> {
        let time = value.time.timestamp_millis();
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&time.to_be_bytes());
        bytes.extend_from_slice(&value.version.to_be_bytes());
        bytes.extend_from_slice(value.root_hash.get_bytes().get_bytes());
        bytes.extend_from_slice(value.previous_block_hash.get_bytes().get_bytes());

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

        let winner = signature.gt(signature2);
        debug!("winner {winner:?}");
    }
}
