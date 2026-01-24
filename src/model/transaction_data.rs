use crate::model::address::Address;
use crate::model::Base36;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct TransactionData {
    from: Address,
    to: Address,
    amount: u64,
    /// Unix timestamp in seconds
    timestamp: u64,
}

impl TransactionData {
    pub fn new(from: &Address, to: &Address, amount: u64, timestamp: u64) -> Self {
        TransactionData {
            from: from.into(),
            to: to.into(),
            amount,
            timestamp,
        }
    }
}

impl From<&TransactionData> for Vec<u8> {
    fn from(value: &TransactionData) -> Self {
        value.as_bytes().to_vec()
    }
}

impl From<&TransactionData> for Base36 {
    fn from(transaction_data: &TransactionData) -> Self {
        let bytes: Vec<u8> = transaction_data.into();
        Base36::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::key::Key;
    use crate::utils::FromBase36;

    #[test]
    fn test_transaction() {
        let from_address = Address::new(Key::from_base36(
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833",
        ));
        let to_address = Address::new(Key::from_base36(
            "1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso",
        ));

        let transaction = TransactionData {
            from: from_address,
            to: to_address,
            amount: 1,
            timestamp: 0,
        };

        let transaction_bytes: Base36 = (&transaction).into();

        println!("transaction_bytes: {}", &transaction_bytes);
    }

    #[test]
    fn test_from_transaction() {
        let from_address = Address::new(Key::from_base36(
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833",
        ));
        let to_address = Address::new(Key::from_base36(
            "1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso",
        ));

        let transaction = TransactionData {
            from: from_address,
            to: to_address,
            amount: 1,
            timestamp: 0,
        };

        let transaction_bytes: Base36 = (&transaction).into();

        println!("transaction_bytes: {}", &transaction_bytes);
    }
}
