use std::io::{Read, Write};

use crate::model::{
    address::Address, signature::Signature, transaction_data::TransactionData, Base36, Hash,
    PrivateAddress,
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
    pub fn new(private_address: &PrivateAddress, to_address: &Address, amount: u64) -> Self {
        let transaction =
            TransactionData::new(private_address.get_address(), to_address, amount, 0);

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
        let db_path = crate::config::CONFIG.with(|config| config.borrow().get_db_path().clone());
        let file_path = format!("{}/transaction.bin", db_path);

        std::fs::create_dir_all(&db_path).unwrap();

        let bytes: Vec<u8> = self.as_bytes().to_vec();
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(&bytes).unwrap();
    }

    pub fn load() -> Self {
        let db_path = crate::config::CONFIG.with(|config| {
            let config_ref = config.borrow();
            let db_path = config_ref.get_db_path();
            db_path.clone()
        });
        let file_path = format!("{}/transaction.bin", db_path);

        std::fs::create_dir_all(&db_path).unwrap();

        let mut file = std::fs::File::open(file_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        Self::read_from(&bytes).unwrap()
    }
}

impl From<&Transaction> for Base36 {
    fn from(transaction: &Transaction) -> Self {
        let bytes: Vec<u8> = transaction.as_bytes().to_vec();
        Base36::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::{private_address::PrivateAddress, Base36};
    use zerocopy::AsBytes;

    #[test]
    fn test_signing_transaction() {
        let from_private_address = PrivateAddress::default();
        let to_private_address = PrivateAddress::default();

        let transaction =
            Transaction::new(&from_private_address, &to_private_address.get_address(), 1);

        let transaction_b36: Base36 = (&transaction).into();
        println!("transaction: {}", transaction_b36);
    }

    #[test]
    fn test_verifying_transaction() {
        let from_private_address = PrivateAddress::default();
        let to_private_address = PrivateAddress::default();

        let transaction =
            Transaction::new(&from_private_address, &to_private_address.get_address(), 1);

        let transaction_b36: Base36 = (&transaction).into();
        println!("transaction: {}", transaction_b36);

        let bytes = transaction.as_bytes();

        let parsed_transaction = Transaction::read_from(bytes).unwrap();

        let verified = parsed_transaction.verify(&from_private_address.get_address());

        println!("verified: {:?}", verified);
    }

    #[test]
    fn test_save_transaction() {
        let from_private_address = PrivateAddress::default();
        let to_private_address = PrivateAddress::default();

        let transaction =
            Transaction::new(&from_private_address, &to_private_address.get_address(), 1);

        let transaction_b36: Base36 = (&transaction).into();
        println!("transaction: {}", transaction_b36);

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
            crate::config::CONFIG.with(|config| config.borrow().get_db_path().clone())
        );

        transaction.save();

        let loaded_transaction = Transaction::load();

        let loaded_transaction_b36: Base36 = (&loaded_transaction).into();
        println!("loaded_transaction: {}", loaded_transaction_b36);

        let verified = loaded_transaction.verify(&from_private_address.get_address());

        println!("verified: {:?}", verified);
    }
}
