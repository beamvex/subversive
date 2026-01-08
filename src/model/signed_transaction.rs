use crate::model::{signature::Signature, transaction::Transaction};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Signature,
}

mod tests {
    use crate::model::address::Address;
    use crate::model::transaction::Transaction;
    use crate::crypto::sign;
    use zerocopy::AsBytes;
    use crate::utils::{base36_to_bytes_32, bytes_to_base36};

    #[test]
    fn test_transaction() {
        let private_key_bytes1 = base36_to_bytes_32("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");
        let private_key_bytes2 = base36_to_bytes_32("1f1uklaakeqg1xhjlvnihhi5ipyu4kgoj7pq0uqkhajovr0pso");

        let public_key1: [u8; 32] = private_key_bytes1
            .as_slice()
            .try_into()
            .expect("base36_to_bytes_32 must return 32 bytes");

        let address1 = Address { public_key: public_key1 };
        
        let public_key2: [u8; 32] = private_key_bytes2
            .as_slice()
            .try_into()
            .expect("base36_to_bytes_32 must return 32 bytes");

        let address2 = Address { public_key: public_key2 };
        

        let transaction = Transaction {
            from: address1,
            to: address2,
            amount: 1,
            timestamp: 0,
        };

        let transaction_bytes = transaction.as_bytes();
        println!("transaction_bytes: {}", bytes_to_base36(&transaction_bytes));

        let signature_bytes = sign(&transaction_bytes, &private_key_bytes1);
        println!("signature_bytes: {}", bytes_to_base36(&signature_bytes));
    }

}