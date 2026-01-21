use crate::model::{block_data::BlockData, signature::Signature};
use crate::utils::ToBase36;
use zerocopy::AsBytes;

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

impl From<&mut Block> for Vec<u8> {
    fn from(value: &mut Block) -> Vec<u8> {
        let mut result = Vec::new();
        let data_bytes: Vec<u8> = value.get_data().into();
        result.extend_from_slice(&data_bytes);
        result.extend_from_slice(value.get_signature().as_bytes());
        result
    }
}

impl ToBase36 for Block {
    fn to_base36(&self) -> String {
        let bytes: Vec<u8> = self.into();
        base36::encode(&bytes)
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
        const TRANSACTION_COUNT: usize = 20;

        let from_private_address = PrivateAddress::default();
        let to_private_address = PrivateAddress::default();

        for _ in 0..TRANSACTION_COUNT {
            let transaction = create_transaction(&from_private_address, &to_private_address);
            block.get_data().get_transactions().push(transaction);
        }
        assert_eq!(block.get_data().get_transactions().len(), TRANSACTION_COUNT);
        println!("Time taken: {} seconds", start.elapsed().as_secs());

        let block_bytes: Vec<u8> = (&mut block).into();
        println!("Block bytes: {}", block_bytes.len());

        let block_base36 = block.to_base36();
        println!("Block base36: {}", block_base36);
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
