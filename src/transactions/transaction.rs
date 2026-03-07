use base_xx::{byte_vec::Encodable, ByteVec, SerialiseError};
use chrono::{DateTime, Utc};
use slahasher::Hashable;

use crate::address::public_address::PublicAddress;
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
        let mut result = Vec::new();

        let from_rc = Rc::clone(&value.from);
        let to_rc = Rc::clone(&value.to);

        let from_bytes = Self::try_from(&*from_rc)?;
        let to_bytes = Self::try_from(&*to_rc)?;

        result.extend_from_slice(from_bytes.get_bytes());
        result.extend_from_slice(to_bytes.get_bytes());
        result.extend_from_slice(&value.amount.to_le_bytes());
        result.extend_from_slice(&value.timestamp.timestamp().to_le_bytes());
        Ok(Self::new(result))
    }
}

impl Hashable for Transaction {}
impl Encodable for Transaction {}

#[cfg(test)]
mod tests {

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
        debug!("transaction_bytes: {transaction_bytes:?}");
    }
}
