use crate::model::Hash;
use sha3::{Digest, Keccak256};

impl Hash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Keccak256::default();
        hasher.update(bytes);
        let result = hasher.finalize();
        Hash::new(result.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::address::Address;
    use crate::model::key::Key;

    use crate::model::transaction::Transaction;
    use crate::utils::{FromBase36, ToBase36};

    #[test]
    fn test_hash() {
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

        let bytes: Vec<u8> = (&transaction).into();
        let hash = Hash::from_bytes(&bytes);

        println!("hash: {}", hash.to_base36());
    }
}
