use crate::model::Base36;
use crate::model::{block_header, signature::Signature};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct Block {
    header: block_header::BlockHeader,
    signature: Signature,
}

impl Block {
    pub fn new(header: block_header::BlockHeader, signature: Signature) -> Self {
        Self { header, signature }
    }

    pub fn get_header(&self) -> &block_header::BlockHeader {
        &self.header
    }

    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }
}

impl From<&Block> for Vec<u8> {
    fn from(value: &Block) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
}

impl From<&Block> for Base36 {
    fn from(value: &Block) -> Self {
        let bytes: Vec<u8> = value.into();
        Base36::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::block_header::BlockHeader;
    use crate::model::hash::Hash;
    use crate::model::Base36;
    use crate::utils::FromBase36;
    use zerocopy::AsBytes;

    #[test]
    fn test_block() {
        let data = BlockHeader::new(
            [1],
            [0; 8],
            Hash::from_base36("456"),
            1234567890,
            Hash::from_base36("789"),
        );
        let signature = Signature::from_base36("012");
        let block = Block::new(data, signature);
        assert_eq!(block.get_header().get_timestamp(), 1234567890);

        assert_eq!(
            block.get_header().get_previous_hash().as_bytes(),
            Hash::from_base36("789").as_bytes()
        );
        assert_eq!(
            block.get_header().get_id().as_bytes(),
            Hash::from_base36("456").as_bytes()
        );

        let block_bytes: Vec<u8> = (&block).into();
        println!("Block bytes: {}", block_bytes.len());

        let block_base36: Base36 = (&block).into();
        println!("Block base36: {}", block_base36.get_string());
    }
}
