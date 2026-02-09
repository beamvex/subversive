#[derive(Debug)]
pub struct SerialiseError {
    message: String,
}

impl SerialiseError {
    #[must_use]
    pub const fn new(message: String) -> Self {
        Self { message }
    }

    #[must_use]
    pub const fn get_message(&self) -> &String {
        &self.message
    }
}
