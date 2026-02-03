pub trait AsBytes {
    type Error;

    /// Convert `self` into a byte vector.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `self` cannot be represented as bytes.
    fn try_as_bytes(&self) -> Result<Vec<u8>, Self::Error>;
}
