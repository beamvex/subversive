use crate::{
    model::{
        address::Address, signature::Signature, transaction_data::TransactionData, PrivateAddress,
    },
    utils::ToBase36,
};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct Transaction {
    transaction: TransactionData,
    signature: Signature,
}

impl Transaction {
    pub fn new(transaction: TransactionData, private_address: &PrivateAddress) -> Self {
        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        Self {
            transaction,
            signature,
        }
    }

    pub fn verify(&self, public_address: &Address) -> bool {
        let bytes: Vec<u8> = (&self.transaction).into();
        public_address.verify(&bytes, &self.signature)
    }
}

impl ToBase36 for Transaction {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::address::Address;
    use crate::model::key::Key;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;
    use crate::utils::{FromBase36, ToBase36};
    use zerocopy::AsBytes;

    #[test]
    fn test_signing_transaction() {
        let private_address = PrivateAddress::new();
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

        let transaction = Transaction::new(transaction, &private_address);

        println!("transaction: {}", transaction.to_base36());
    }

    #[test]
    fn test_verifying_transaction() {
        let private_address = PrivateAddress::new();
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

        let transaction = Transaction::new(transaction, &private_address);

        println!("transaction: {}", transaction.to_base36());

        let bytes = transaction.as_bytes();

        let parsed_transaction = Transaction::read_from(bytes).unwrap();

        let verified = parsed_transaction.verify(&private_address.get_address());

        println!("verified: {:?}", verified);
    }
}
