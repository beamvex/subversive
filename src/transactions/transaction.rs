use base_xx::{byte_vec::Encodable, encoded_string::Decodable, ByteVec, SerialiseError};
use chrono::{DateTime, Utc};
use slahasher::Hashable;

use crate::{address::public_address::PublicAddress, serialise::RLEByteVec};
use std::rc::Rc;

/// A transaction between two public addresses.
#[derive(Debug, Default)]
pub struct Transaction {
    from: Rc<PublicAddress>,
    to: Rc<PublicAddress>,
    amount: u64,
    /// Unix timestamp in seconds
    timestamp: DateTime<Utc>,
}

impl Transaction {
    /// Creates a new transaction.
    #[must_use]
    pub const fn new(
        from: Rc<PublicAddress>,
        to: Rc<PublicAddress>,
        amount: u64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            from,
            to,
            amount,
            timestamp,
        }
    }

    /// Get from addtess
    #[must_use]
    pub const fn get_from(&self) -> &Rc<PublicAddress> {
        &self.from
    }

    /// get to address
    #[must_use]
    pub const fn get_to(&self) -> &Rc<PublicAddress> {
        &self.to
    }

    /// Get amount
    #[must_use]
    pub const fn get_amount(&self) -> u64 {
        self.amount
    }

    /// Get timestamp
    #[must_use]
    pub const fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

impl TryFrom<&Transaction> for ByteVec {
    type Error = SerialiseError;

    fn try_from(value: &Transaction) -> Result<Self, Self::Error> {
        let mut result = RLEByteVec::default();

        let from_rc = Rc::clone(&value.from);
        let to_rc = Rc::clone(&value.to);

        let from_bytes = Self::try_from(&*from_rc)?;
        let to_bytes = Self::try_from(&*to_rc)?;

        result.add_data(Rc::new(from_bytes));
        result.add_data(Rc::new(to_bytes));
        result.add_data(Rc::new(Self::new(value.amount.to_le_bytes().to_vec())));
        result.add_data(Rc::new(Self::new(
            value.timestamp.timestamp().to_le_bytes().to_vec(),
        )));
        Self::try_from(&result)
    }
}

impl TryFrom<ByteVec> for Transaction {
    type Error = SerialiseError;

    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        let rle = RLEByteVec::try_from(value)?;
        let rle = rle.get_data();
        let from_bytes = rle.first();
        let to_bytes = rle.get(1);
        let amount_bytes = rle.get(2);
        let timestamp_bytes = rle.get(3);

        let from = PublicAddress::try_from(
            *from_bytes.ok_or(SerialiseError::new("Missing from field".to_string()))?,
        )?;
        let to = PublicAddress::try_from(
            *to_bytes.ok_or(SerialiseError::new("Missing to field".to_string()))?,
        )?;
        let amount = amount_bytes
            .ok_or(SerialiseError::new("Missing amount field".to_string()))?
            .try_into()?;
        let timestamp = timestamp_bytes
            .ok_or(SerialiseError::new("Missing timestamp".to_string()))?
            .try_into()?;

        Ok(Self {
            from: Rc::new(from),
            to: Rc::new(to),
            amount,
            timestamp,
        })
    }
}

impl Hashable for Transaction {}
impl Encodable for Transaction {}
impl Decodable for Transaction {}

#[cfg(test)]
mod tests {

    use crate::transactions::TransactionSignature;

    use super::*;
    use simple_sign::Ed25519Signer;
    use slogger::debug;

    #[test]
    fn test_transaction() {
        let private_address = Ed25519Signer::new_random();

        let public_address =
            PublicAddress::try_from(&private_address).unwrap_or_else(|_| unreachable!());
        debug!("public_address: {public_address:?}");

        let private_address2 = Ed25519Signer::new_random();

        let public_address2 =
            PublicAddress::try_from(&private_address2).unwrap_or_else(|_| unreachable!());
        debug!("public_address2: {public_address2:?}");

        let transaction = Transaction::new(
            Rc::new(public_address),
            Rc::new(public_address2),
            100,
            Utc::now(),
        );
        debug!("transaction: {transaction:?}");

        let transaction_bytes = ByteVec::try_from(&transaction).unwrap_or_else(|e| {
            panic!("Failed to serialize transaction: {e}");
        });
        debug!("transaction_bytes: {transaction_bytes:#?}");

        let transaction_hash = transaction.try_hash(slahasher::HashAlgorithm::KECCAK512);
        debug!("transaction_hash: {transaction_hash:#?}");

        let signature = TransactionSignature::new(&transaction, &private_address)
            .unwrap_or_else(|e| unreachable!("Error {e}"));
        debug!("signature:\n {signature:#?}");
    }
}
