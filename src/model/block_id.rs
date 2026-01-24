use crate::model::hash::Hash;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes)]
pub struct BlockId {
    hash: Hash,
    ts: u64,
}

impl Default for BlockId {
    fn default() -> Self {
        let ts: u64 = Self::get_last_time_block()
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after unix epoch")
            .as_secs();

        let hash = Hash::from(ts);
        Self::new(hash, ts)
    }
}

impl BlockId {
    pub fn new(hash: Hash, ts: u64) -> Self {
        Self { hash, ts }
    }

    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }

    pub fn to_last_time_block(t: SystemTime) -> SystemTime {
        let secs = t
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after unix epoch")
            .as_secs();

        let block_secs = (secs / 600) * 600;
        UNIX_EPOCH + Duration::from_secs(block_secs)
    }

    pub fn get_last_time_block() -> SystemTime {
        Self::to_last_time_block(SystemTime::now())
    }
}

impl From<&BlockId> for Vec<u8> {
    fn from(value: &BlockId) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_time_block() {
        let base = UNIX_EPOCH + Duration::from_secs(600 * 12345);
        let t = base + Duration::from_secs(5 * 60);
        let last_time_block = BlockId::to_last_time_block(t);

        let iso = chrono::DateTime::<chrono::Utc>::from(last_time_block).to_rfc3339();
        println!("last_time_block: {}", iso);

        assert_eq!(last_time_block, base);
    }
}
