pub trait FromBytes {
    type Error;

    /// Construct `Self` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the provided `bytes` do not encode a valid `Self`.
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
