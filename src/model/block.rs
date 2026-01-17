use crate::model::transaction::Transaction;

pub struct Block {
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;
    use crate::utils::ToBase36;
    use zerocopy::AsBytes;

    #[test]
    fn test_block() {
        let mut block = Block {
            timestamp: 1234567890,
            transactions: vec![],
            previous_hash: "previous_hash".to_string(),
            hash: "hash".to_string(),
        };
        assert_eq!(block.timestamp, 1234567890);

        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.hash, "hash");

        const TRANSACTION_COUNT: usize = 50000;

        for _ in 0..TRANSACTION_COUNT {
            let transaction = create_transaction();
            block.transactions.push(transaction);
        }
        assert_eq!(block.transactions.len(), TRANSACTION_COUNT);
    }

    fn create_transaction() -> Transaction {
        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        let transaction = TransactionData::new(
            from_private_address.get_address(),
            to_private_address.get_address(),
            1,
            0,
        );

        let transaction = Transaction::new(transaction, &from_private_address);

        //println!("transaction: {}", transaction.to_base36());
        transaction
    }
}
