use crate::model::{block::Block, signature::Signature};

pub struct SignedBlock {
    pub block: Block,
    pub signature: Signature,
}
