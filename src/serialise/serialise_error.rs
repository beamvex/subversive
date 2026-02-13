/// Error type for serialization operations.
///
/// This type represents errors that can occur during serialization and
/// deserialization of data structures.
#[derive(Debug)]
pub struct SerialiseError {
    /// The error message describing what went wrong
    message: String,
}

impl SerialiseError {
    /// Creates a new serialization error with the given message.
    ///
    /// # Arguments
    /// * `message` - A description of what went wrong during serialization
    #[must_use]
    pub const fn new(message: String) -> Self {
        Self { message }
    }

    /// Returns a reference to the error message.
    #[must_use = "This returns the error message string and does nothing if unused"]
    pub const fn get_message(&self) -> &String {
        &self.message
    }
}
