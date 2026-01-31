use crate::address::key::Key;
use crate::address::public_address::PublicAddress;

pub struct PrivateAddress {
    private_key: Key,
    public_address: PublicAddress,
}

impl PrivateAddress {
    pub fn get_private_key(&self) -> &Key {
        &self.private_key
    }

    pub fn get_address(&self) -> &PublicAddress {
        &self.public_address
    }
}

/*
#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::transaction_data::TransactionData;

    #[test]
    fn test_generate_key() {
        let private_address = PrivateAddress::default();

        let private_address_b36: Base36 = (&private_address).into();
        let public_address_b36: Base36 = private_address.get_address().into();

        println!("1. private_key_b36: {}", private_address_b36);
        println!("2. public_key_b36: {}", public_address_b36);

        assert_eq!(private_address.get_private_key().get_bytes().len(), 32);
        assert_eq!(
            private_address
                .get_address()
                .get_public_key()
                .get_bytes()
                .len(),
            32
        );

        println!("private_address_bytes: {}", private_address_b36);
    }

    #[test]
    fn test_sign() {
        let from_private_address = PrivateAddress::default();
        let from_address = from_private_address.get_address();
        let to_private_address = PrivateAddress::default();
        let to_address = to_private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let signature = from_private_address.sign(&bytes);

        let signature_b36: Base36 = (&signature).into();
        println!("signature: {}", signature_b36);
    }
}
*/
