use base_xx::{ByteVec, SerialiseError};
use slahasher::Hashable;
use std::time::{Duration, SystemTime};

/// block in a chain
pub struct Block {
    duration: Duration,
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

impl TryFrom<ByteVec> for Block {
    type Error = SerialiseError;
    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryFrom<&Block> for ByteVec {
    type Error = SerialiseError;
    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        let duration = value.duration.as_millis().to_be_bytes().to_vec();
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&duration);

        Ok(Self::new(bytes))
    }
}

impl Hashable for Block {}

mod tests {
    use super::*;
    use simple_sign::{Ed25519Signer, Signature, Signer};
    use slahasher::{Hash, HashAlgorithm};
    use slogger::debug;

    #[test]
    fn test_play() {
        let private_key = Ed25519Signer::new_random();

        //let bytes = base_xx::ByteVec::new();

        let block = Block::default();

        let signature = match block.try_hash(HashAlgorithm::KECCAK512) {
            Ok(hash) => {
                let signature = private_key.sign(&hash);
                match signature {
                    Ok(signature) => signature,
                    Err(e) => {
                        eprintln!("Error: {e}");
                        assert!(false);
                        Signature::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                assert!(false);
                Signature::default()
            }
        };

        debug!("signature {signature:?}");
    }
}
