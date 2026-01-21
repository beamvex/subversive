use crate::model::{block_data::BlockData, signature::Signature};

pub struct Block {
    data: BlockData,
    signature: Signature,
}

impl Block {
    pub fn new(data: BlockData, signature: Signature) -> Self {
        Self { data, signature }
    }

    pub fn get_data(&mut self) -> &mut BlockData {
        &mut self.data
    }

    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::block_header::BlockHeader;
    use crate::model::hash::Hash;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction::Transaction;
    use crate::utils::FromBase36;
    use zerocopy::AsBytes;

    #[test]
    fn test_block() {
        let data = BlockData::new(
            Hash::from_base36("123"),
            BlockHeader::new(
                [1],
                [0; 8],
                Hash::from_base36("456"),
                1234567890,
                Hash::from_base36("789"),
            ),
            vec![],
        );
        let signature = Signature::from_base36("012");
        let mut block = Block::new(data, signature);
        assert_eq!(block.get_data().get_header().get_timestamp(), 1234567890);

        assert_eq!(
            block.get_data().get_header().get_previous_hash().as_bytes(),
            Hash::from_base36("789").as_bytes()
        );
        assert_eq!(
            block.get_data().get_hash().as_bytes(),
            Hash::from_base36("123").as_bytes()
        );

        let start = std::time::Instant::now();
        const TRANSACTION_COUNT: usize = 10;

        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        for _ in 0..TRANSACTION_COUNT {
            let transaction = create_transaction(&from_private_address, &to_private_address);
            block.get_data().get_transactions().push(transaction);
        }
        assert_eq!(block.get_data().get_transactions().len(), TRANSACTION_COUNT);
        println!("Time taken: {} seconds", start.elapsed().as_secs());
    }

    fn create_transaction(
        from_private_address: &PrivateAddress,
        to_private_address: &PrivateAddress,
    ) -> Transaction {
        let transaction =
            Transaction::new(&from_private_address, &to_private_address.get_address(), 1);

        //println!("transaction: {}", transaction.to_base36());
        transaction
    }
}
