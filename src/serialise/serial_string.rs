use crate::serialise::SerialiseType;

#[derive(Debug)]
pub struct SerialString {
    serialise_type: SerialiseType,
    string: String,
}

impl SerialString {
    #[must_use]
    pub const fn new(serialise_type: SerialiseType, string: String) -> Self {
        Self {
            serialise_type,
            string,
        }
    }

    #[must_use]
    pub const fn get_serialise_type(&self) -> SerialiseType {
        self.serialise_type
    }

    #[must_use]
    pub const fn get_string(&self) -> &String {
        &self.string
    }
}

impl std::fmt::Display for SerialString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
