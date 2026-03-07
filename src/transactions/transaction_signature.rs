use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use simple_sign::{Ed25519Signer, Signature, SignatureError, Signer};
use slahasher::{Hash, HashAlgorithm, Hashable};

use crate::transactions::Transaction;

/// Transaction signature
#[derive(Debug, PartialEq, Eq)]
pub struct TransactionSignature {
    /// Hash/Id of the transaction
    id: Hash,

    /// Signature of the transaction
    signature: Signature,
}

impl TransactionSignature {
    /// Create a new transaction signature
    ///
    /// # Arguments
    /// * `transaction` - The transaction to sign
    /// * `signature` - The signature
    ///
    /// # Returns
    /// * `Result<Self, SignatureError>` - The transaction signature or an error
    ///
    /// # Errors
    /// * `SignatureError` - If the transaction cannot be hashed
    pub fn new(transaction: &Transaction, signer: &Ed25519Signer) -> Result<Self, SignatureError> {
        let id = match transaction.try_hash(HashAlgorithm::KECCAK512) {
            Ok(id) => id,
            Err(e) => {
                return Err(SignatureError::new(format!(
                    "Failed to hash transaction: {e}"
                )))
            }
        };

        let signature = match signer.sign(&id) {
            Ok(signature) => signature,
            Err(e) => {
                return Err(SignatureError::new(format!(
                    "Failed to sign transaction: {e}"
                )))
            }
        };

        Ok(Self { id, signature })
    }
}

impl TryFrom<&TransactionSignature> for ByteVec {
    type Error = SerialiseError;

    fn try_from(value: &TransactionSignature) -> Result<Self, Self::Error> {
        let id_bytes = Self::try_from(&value.id)?;
        let signature_bytes = Self::try_from(&value.signature)?;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(id_bytes.get_bytes());
        bytes.extend_from_slice(signature_bytes.get_bytes());
        Ok(Self::new(bytes))
    }
}

impl Encodable for TransactionSignature {}
