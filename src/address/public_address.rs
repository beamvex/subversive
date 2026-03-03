//! Public address type and byte encoding/decoding.

use base_xx::{byte_vec::Encodable, ByteVec};

/// A public address.
///
/// Encodes to bytes as: `[version][public_key_bytes...]`.
pub struct PublicAddress {
    public_key: ByteVec,
    version: u8,
}

impl PublicAddress {
    #[must_use]
    /// Creates a new `PublicAddress` with the default version.
    pub const fn new(public_key: ByteVec) -> Self {
        Self {
            public_key,
            version: 1,
        }
    }

    #[must_use]
    /// Returns the public key bytes.
    pub const fn get_public_key(&self) -> &ByteVec {
        &self.public_key
    }

    #[must_use]
    /// Returns the address version byte.
    pub const fn get_version(&self) -> u8 {
        self.version
    }
}

impl TryFrom<&PublicAddress> for ByteVec {
    type Error = base_xx::SerialiseError;
    fn try_from(value: &PublicAddress) -> Result<Self, Self::Error> {
        let mut bytes = Vec::with_capacity(1 + value.public_key.get_bytes().len());
        bytes.push(value.version);
        bytes.extend_from_slice(value.public_key.get_bytes());
        Ok(Self::new(bytes))
    }
}

impl TryFrom<ByteVec> for PublicAddress {
    type Error = base_xx::SerialiseError;
    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        let bytes = value.get_bytes();
        if bytes.is_empty() {
            return Err(base_xx::SerialiseError::new(
                "PublicAddress requires at least 1 byte for version".to_string(),
            ));
        }

        let version = bytes[0];
        let public_key = ByteVec::new(bytes[1..].to_vec());
        Ok(Self::new(public_key))
    }
}

impl Encodable for PublicAddress {}

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
