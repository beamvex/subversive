/// A trait for types that can be constructed from bytes.
///
/// This trait should be implemented by types that can be deserialized from
/// a byte representation. The associated `Error` type specifies what kind
/// of errors can occur during deserialization.
pub trait FromBytes {
    /// The type of error that can occur during deserialization.
    type Error;

    /// Attempts to create an instance of `Self` from a byte slice.
    ///
    /// # Arguments
    /// * `bytes` - The byte slice to deserialize from
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if deserialization fails.
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
