use std::io::{Read, Write};

use crate::{
    model::{
        address::Address, signature::Signature, transaction_data::TransactionData, Hash,
        PrivateAddress,
    },
    utils::ToBase36,
};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct Transaction {
    id: Hash,
    data: TransactionData,
    signature: Signature,
}

impl Transaction {
    pub fn new(transaction: TransactionData, private_address: &PrivateAddress) -> Self {
        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        Self {
            id: Hash::from_bytes(&bytes),
            data: transaction,
            signature,
        }
    }

    pub fn verify(&self, public_address: &Address) -> bool {
        let bytes: Vec<u8> = (&self.data).into();
        public_address.verify(&bytes, &self.signature)
    }

    pub fn save(&self) {
        let db_path = crate::config::CONFIG.with(|config| config.borrow().get_db_path());
        let file_path = format!("{}/transaction.bin", db_path);

        std::fs::create_dir_all(&db_path).unwrap();

        let bytes: Vec<u8> = self.as_bytes().to_vec();
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(&bytes).unwrap();
    }

    pub fn load() -> Self {
        let db_path = crate::config::CONFIG.with(|config| config.borrow().get_db_path());
        let file_path = format!("{}/transaction.bin", db_path);

        std::fs::create_dir_all(db_path).unwrap();

        let mut file = std::fs::File::open(file_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        Self::read_from(&bytes).unwrap()
    }
}

impl ToBase36 for Transaction {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::private_address::PrivateAddress;
    use crate::model::transaction_data::TransactionData;
    use crate::utils::ToBase36;
    use zerocopy::AsBytes;

    #[test]
    fn test_signing_transaction() {
        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        let transaction = TransactionData::new(
            from_private_address.get_address(),
            to_private_address.get_address(),
            1,
            0,
        );

        let transaction = Transaction::new(transaction, &from_private_address);

        println!("transaction: {}", transaction.to_base36());
    }

    #[test]
    fn test_verifying_transaction() {
        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        let transaction = TransactionData::new(
            from_private_address.get_address(),
            to_private_address.get_address(),
            1,
            0,
        );

        let transaction = Transaction::new(transaction, &from_private_address);

        println!("transaction: {}", transaction.to_base36());

        let bytes = transaction.as_bytes();

        let parsed_transaction = Transaction::read_from(bytes).unwrap();

        let verified = parsed_transaction.verify(&from_private_address.get_address());

        println!("verified: {:?}", verified);
    }

    #[test]
    fn test_save_transaction() {
        let from_private_address = PrivateAddress::new();
        let to_private_address = PrivateAddress::new();

        let transaction = TransactionData::new(
            from_private_address.get_address(),
            to_private_address.get_address(),
            1,
            0,
        );

        let transaction = Transaction::new(transaction, &from_private_address);

        println!("transaction: {}", transaction.to_base36());

        let bytes = transaction.as_bytes();

        let parsed_transaction = Transaction::read_from(bytes).unwrap();

        let verified = parsed_transaction.verify(&from_private_address.get_address());

        println!("verified: {:?}", verified);

        let db_path = "./tmp/db_{}";
        crate::config::CONFIG.with(|config| {
            config.borrow_mut().set_db_path(db_path);
        });
        println!(
            "db_path: {}",
            crate::config::CONFIG.with(|config| config.borrow().get_db_path())
        );

        transaction.save();

        let loaded_transaction = Transaction::load();

        println!("loaded_transaction: {}", loaded_transaction.to_base36());

        let verified = loaded_transaction.verify(&from_private_address.get_address());

        println!("verified: {:?}", verified);
    }
}
