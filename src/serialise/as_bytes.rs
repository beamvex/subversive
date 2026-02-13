/// A trait for types that can be converted to bytes.
///
/// This trait should be implemented by types that can be serialized into
/// a byte representation. The associated `Error` type specifies what kind
/// of errors can occur during serialization.
pub trait AsBytes {
    /// The type of error that can occur during serialization.
    type Error;

    /// Attempts to convert this value to its byte representation.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if serialization fails.
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error>;
}
