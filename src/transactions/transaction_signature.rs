use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use simple_sign::{Ed25519Signer, Signature, SignatureError, Signer};
use slahasher::{Hash, HashAlgorithm};
use std::sync::Arc;

use crate::transactions::Transaction;

/// Transaction signature
#[derive(Debug, PartialEq, Eq)]
pub struct TransactionSignature {
    /// Hash/Id of the transaction
    id: Arc<Hash>,

    /// Signature of the transaction
    signature: Arc<Signature>,
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
    pub fn new(
        transaction: &Transaction,
        signer: Arc<Ed25519Signer>,
    ) -> Result<Self, SignatureError> {
        let bytes = base_xx::ByteVec::try_from(transaction)
            .map_err(|e| SignatureError::new(format!("Failed to serialize transaction: {e}")))?;

        let id = Hash::try_hash(Arc::new(bytes), HashAlgorithm::KECCAK512)
            .map_err(|e| SignatureError::new(format!("Failed to hash transaction: {e}")))?;

        let signature = signer
            .sign(Arc::clone(&id))
            .map_err(|e| SignatureError::new(format!("Failed to sign transaction: {e}")))?;

        Ok(Self { id, signature })
    }
}

impl TryFrom<&TransactionSignature> for ByteVec {
    type Error = SerialiseError;

    fn try_from(value: &TransactionSignature) -> Result<Self, Self::Error> {
        let id_bytes =
            <Hash as base_xx::byte_vec::TryIntoByteVec>::try_into_byte_vec(Arc::clone(&value.id))?;
        let signature_bytes = <Signature as base_xx::byte_vec::TryIntoByteVec>::try_into_byte_vec(
            Arc::clone(&value.signature),
        )?;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(id_bytes.get_bytes());
        bytes.extend_from_slice(signature_bytes.get_bytes());
        Ok(Self::new(bytes.into()))
    }
}

impl base_xx::byte_vec::TryIntoByteVec for TransactionSignature {
    fn try_into_byte_vec(value: Arc<Self>) -> Result<Arc<ByteVec>, SerialiseError> {
        Ok(Arc::new(ByteVec::try_from(value.as_ref())?))
    }
}

impl Encodable for TransactionSignature {}
