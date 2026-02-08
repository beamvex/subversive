use crate::{
    address::key::Key,
    hashable, serialisable,
    serialise::{AsBytes, Bytes, FromBytes, StructType},
};

pub struct PublicAddress {
    public_key: Key,
}

impl PublicAddress {
    #[must_use]
    pub const fn new(public_key: Key) -> Self {
        Self { public_key }
    }

    #[must_use]
    pub const fn get_public_key(&self) -> &Key {
        &self.public_key
    }
}

impl AsBytes for PublicAddress {
    type Error = ();
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.public_key.try_as_bytes().unwrap());
        Ok(bytes)
    }
}

impl FromBytes for PublicAddress {
    type Error = ();
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes = bytes.to_vec();
        Ok(Self::new(Key::try_from_bytes(&bytes).unwrap()))
    }
}

impl TryFrom<PublicAddress> for Bytes {
    type Error = &'static str;
    fn try_from(value: PublicAddress) -> Result<Self, Self::Error> {
        Ok(Self::new(
            StructType::ADDRESS,
            value.try_as_bytes().unwrap(),
        ))
    }
}

serialisable!(PublicAddress);
hashable!(PublicAddress);

/*
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify() {
        let private_address = PrivateAddress::default();
        let from_address = private_address.get_address();
        let to_address = private_address.get_address();

        let transaction = TransactionData::new(from_address, to_address, 1, 0);

        let bytes: Vec<u8> = (&transaction).into();
        let signature = private_address.sign(&bytes);

        let from_address = private_address.get_address();
        let verified = from_address.verify(&bytes, &signature);

        assert!(verified);
    }

    #[test]
    fn test_address() {
        let public_key = Key::from_base36("3375t72oexdn8n814mi1z8yjpubm9yy1uxz1f9o1hpz0qye833");

        let address = Address::new(public_key);

        let address_bytes: Base36 = (&address).into();

        crate::debug!("address_bytes: {}", address_bytes);
    }
}
*/
