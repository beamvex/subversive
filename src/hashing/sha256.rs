use crate::hashing::{Hash, HashAlgorithm};
use sha2::{Digest, Sha256 as Sha256Impl};

/// SHA-256 hash implementation.
///
/// This type provides methods to create and manipulate SHA-256 hashes.
/// SHA-256 is a cryptographic hash function that produces a 256-bit (32-byte)
/// hash value.
#[derive(Debug)]
pub struct Sha256 {
    /// The raw bytes of the hash value
    hash: Hash,
}

impl Sha256 {
    /// Creates a new SHA-256 hash value.
    ///
    /// # Arguments
    /// * `hash` - The raw hash value
    #[must_use = "This creates a new hash value but does nothing if unused"]
    pub const fn new(hash: Hash) -> Self {
        Self { hash }
    }

    /// Creates a SHA-256 hash from a byte slice.
    ///
    /// # Arguments
    /// * `bytes` - The data to hash
    ///
    /// # Returns
    /// A new SHA-256 hash value containing the hash of the input data
    #[must_use = "This computes a hash value but does nothing if unused"]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256Impl::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let bytes = result.to_vec();

        let hash = Hash::new(HashAlgorithm::SHA256, bytes);
        Self::new(hash)
    }
}

/// Implements SHA-256 hashing for a type that implements `AsBytes`.
///
/// This macro generates an implementation that computes the SHA-256 hash
/// of a value by first converting it to bytes using the `AsBytes` trait.
#[macro_export]
macro_rules! impl_sha256_from_as_bytes {
    ($t:ty) => {
        impl From<&$t> for $crate::hashing::Sha256 {
            fn from(value: &$t) -> Self {
                $crate::hashing::Sha256::from_bytes(&value.try_as_bytes().unwrap())
            }
        }
    };
}

impl From<Sha256> for Hash {
    fn from(value: Sha256) -> Self {
        value.hash
    }
}

#[cfg(test)]
mod tests {

    use slogger::debug;

    use crate::serialise::{Base36, Bytes, SerialString};

    use super::*;

    #[test]
    pub fn test_sha256() {
        let test = b"this is a really good test";
        let hash: Hash = Sha256::from_bytes(test).into();
        let bytes: Bytes = (&hash).try_into().unwrap();
        let base36: Base36 = bytes.try_into().unwrap();
        let serialised: SerialString = base36.try_into().unwrap();

        assert!(hash.verify(test));
        debug!("sha256 {serialised}");
    }
}
