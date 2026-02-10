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

#[macro_export]
macro_rules! try_from_serial_string {
    ($t:ty) => {
        impl TryFrom<$t> for SerialString {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(value: $t) -> Result<Self, $crate::serialise::SerialiseError> {
                Ok(value.get_serialised())
            }
        }
    };
}

#[macro_export]
macro_rules! try_to_serial_string {
    ($t:ty) => {
        impl TryFrom<SerialString> for $t {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(value: SerialString) -> Result<Self, $crate::serialise::SerialiseError> {
                Ok(Self::new(value))
            }
        }
    };
}
