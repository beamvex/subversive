use std::default;

use crate::model::address::Address;
use crate::utils::ToBase36;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    /// Unix timestamp in seconds
    pub timestamp: u64,
}

impl ToBase36 for Transaction {}

impl From<&Transaction> for Vec<u8> {
    fn from(value: &Transaction) -> Self {
        value.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::transaction::Transaction;
    use crate::utils::{FromBase36, ToBase36};
    use zerocopy::AsBytes;

    #[test]
    fn test_transaction() {
        let from_address = Address::new(Key::from_base36(
            "3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833",
        ));
        let to_address = Address::new(Key::from_base36(
            "1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso",
        ));

        let transaction = Transaction {
            from: from_address,
            to: to_address,
            amount: 1,
            timestamp: 0,
        };

        let transaction_bytes = transaction.to_base36();

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

        let transaction = Transaction {
            from: from_address,
            to: to_address,
            amount: 1,
            timestamp: 0,
        };

        let transaction_bytes: Vec<u8> = (&transaction).into();

        println!("transaction_bytes: {}", &transaction_bytes.len());
    }
}
