use crate::model::{transaction::Transaction, Hash, Signature};

pub struct BlockHeader {
    version: [u8; 1],
    algo: [u8; 8],
    id: Hash,
    timestamp: u64,
    previous_hash: Hash,
}

pub struct BlockData {
    hash: Hash,
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

pub struct Block {
    data: BlockData,
    signature: Signature,
}

impl BlockHeader {
    pub fn new(
        version: [u8; 1],
        algo: [u8; 8],
        id: Hash,
        timestamp: u64,
        previous_hash: Hash,
    ) -> Self {
        Self {
            version,
            algo,
            id,
            timestamp,
            previous_hash,
        }
    }
}

impl BlockData {
    pub fn new(hash: Hash, header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            hash,
            header,
            transactions,
        }
    }
}

impl Block {
    pub fn new(data: BlockData, signature: Signature) -> Self {
        Self { data, signature }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;
    use crate::utils::{FromBase36, ToBase36};
    use zerocopy::AsBytes;

    #[test]
    fn test_block() {
        let data = BlockData::new(
            Hash::from_base36("hash"),
            BlockHeader::new(
                [1],
                [0; 8],
                Hash::from_base36("id"),
                1234567890,
                Hash::from_base36("previous_hash"),
            ),
            vec![],
        );
        let signature = Signature::from_base36("signature");
        let mut block = Block::new(data, signature);
        assert_eq!(block.data.header.timestamp, 1234567890);

        assert_eq!(
            block.data.header.previous_hash,
            Hash::from_base36("previous_hash")
        );
        assert_eq!(block.data.hash, Hash::from_base36("hash"));

        let start = std::time::Instant::now();
        const TRANSACTION_COUNT: usize = 5500;

        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        for _ in 0..TRANSACTION_COUNT {
            let transaction = create_transaction(&from_private_address, &to_private_address);
            block.data.transactions.push(transaction);
        }
        assert_eq!(block.data.transactions.len(), TRANSACTION_COUNT);
        println!("Time taken: {} seconds", start.elapsed().as_secs());
    }

    fn create_transaction(
        from_private_address: &PrivateAddress,
        to_private_address: &PrivateAddress,
    ) -> Transaction {
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
