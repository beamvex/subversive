//! Public address type and byte encoding/decoding.

use base_xx::{byte_vec::Encodable, ByteVec};
use slahasher::Hashable;

/// A public address.
///
/// Encodes to bytes as: `[version][public_key_bytes...]`.
#[derive(Debug)]
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
        if version != 1 {
            return Err(base_xx::SerialiseError::new(
                "PublicAddress version must be 1".to_string(),
            ));
        }
        let public_key = ByteVec::new(bytes[1..].to_vec());
        Ok(Self::new(public_key))
    }
}

impl Hashable for PublicAddress {}
impl Encodable for PublicAddress {}

#[cfg(test)]
mod tests {

    use base_xx::ByteVec;
    use simple_sign::{Ed25519Signer, Signer};
    use slahasher::HashAlgorithm;
    use slogger::debug;

    use super::*;

    #[test]
    fn test_address() {
        let private_address = Ed25519Signer::new_random();

        let public_address = private_address.get_verifying_key().to_bytes();
        let public_address = ByteVec::new(public_address.to_vec());
        let public_address = PublicAddress::new(public_address);

        debug!("public_address: {public_address:?}");

        let hash = public_address
            .try_hash(HashAlgorithm::KECCAK512)
            .unwrap_or_else(|e| {
                unreachable!("Failed to hash public address {e}");
            });
        debug!("hash: {hash:?}");

        let private_address2 = Ed25519Signer::new_random();

        let signature = private_address2.sign(&hash);
        debug!("signature: {signature:?}");
    }
}
