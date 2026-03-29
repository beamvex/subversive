use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use chrono::{DateTime, Utc};
use simple_sign::{Signature, SignatureError, Signer};
use slahasher::{Hash, HashAlgorithm, Hashable};
use std::sync::Arc;

/// block in a chain
#[derive(Debug)]
pub struct Block {
    time: DateTime<Utc>,
    version: u8,
    root_hash: Arc<Hash>,
    previous_block_hash: Arc<Hash>,
}

impl Block {
    /// Sign the block with the given signer
    ///
    /// # Errors
    ///
    /// Returns an error if the block cannot be hashed or the signer fails to sign
    pub fn try_sign<S: Signer>(&self, signer: Arc<S>) -> Result<Arc<Signature>, SignatureError> {
        let bytes = ByteVec::try_from(self).map_err(|e| SignatureError::new(e.to_string()))?;
        let hash = Hash::try_hash(Arc::new(bytes), HashAlgorithm::KECCAK512)
            .map_err(|e| SignatureError::new(e.to_string()))?;
        signer.sign(hash)
    }
}

impl Default for Block {
    /// Create the "Genesis" block :D
    fn default() -> Self {
        let time = DateTime::default();

        let time_millis = time.timestamp_millis();
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&time_millis.to_be_bytes());
        let bytes = Arc::new(ByteVec::new(bytes.into()));

        let root_hash = Hash::try_hash(Arc::clone(&bytes), HashAlgorithm::KECCAK512)
            .unwrap_or_else(|_| {
                Arc::new(Hash::new(
                    HashAlgorithm::KECCAK512,
                    ByteVec::new(vec![].into()),
                ))
            });
        let previous_block_hash = Hash::try_hash(
            root_hash
                .try_to_byte_vec()
                .unwrap_or_else(|_| Arc::new(ByteVec::new(vec![].into()))),
            HashAlgorithm::KECCAK512,
        )
        .unwrap_or_else(|_| {
            Arc::new(Hash::new(
                HashAlgorithm::KECCAK512,
                ByteVec::new(vec![].into()),
            ))
        });

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

        Ok(Self::new(bytes.into()))
    }
}

impl base_xx::byte_vec::TryIntoByteVec for Block {
    fn try_into_byte_vec(value: Arc<Self>) -> Result<Arc<ByteVec>, SerialiseError> {
        Ok(Arc::new(ByteVec::try_from(value.as_ref())?))
    }
}

impl Hashable for Block {}
impl Encodable for Block {}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_sign::Ed25519Signer;
    use slogger::debug;
    use std::sync::Arc;

    #[test]
    fn test_play() {
        let private_key = Arc::new(Ed25519Signer::new_random());

        let block = Block::default();

        let signature = block.try_sign(Arc::clone(&private_key));
        let signature = match &signature {
            Ok(signature) => signature,
            Err(e) => {
                eprintln!("Error: {e}");
                &Signature::default()
            }
        };

        debug!("signature {signature:?}");
        debug!("block {block:?}");

        let private_key2 = Arc::new(Ed25519Signer::new_random());

        let signature2 = block.try_sign(Arc::clone(&private_key2));
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
