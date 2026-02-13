use crate::serialise::SerialiseType;

/// String representation of serialized data.
///
/// This type represents data that has been serialized into a string format,
/// along with information about which serialization format was used.
#[derive(Debug, Clone)]
pub struct SerialString {
    /// The format used to serialize the data
    serialise_type: SerialiseType,
    /// The serialized string representation
    string: String,
}

impl SerialString {
    /// Creates a new `SerialString` instance.
    ///
    /// # Arguments
    /// * `serialise_type` - The format used to serialize the data
    /// * `string` - The serialized string representation
    #[must_use = "This creates a new SerialString instance but does nothing if unused"]
    pub const fn new(serialise_type: SerialiseType, string: String) -> Self {
        Self {
            serialise_type,
            string,
        }
    }

    /// Returns the format used to serialize the data.
    ///
    /// # Returns
    /// The serialization format.
    #[must_use = "This returns the serialization format but does nothing if unused"]
    pub const fn get_serialise_type(&self) -> SerialiseType {
        self.serialise_type
    }

    /// Returns the serialized string representation.
    ///
    /// # Returns
    /// The serialized string.
    #[must_use = "This returns the serialized string but does nothing if unused"]
    pub const fn get_string(&self) -> &String {
        &self.string
    }
}

impl std::fmt::Display for SerialString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// Implements `TryFrom<SerialString>` for a type.
///
/// This macro generates an implementation that attempts to create an instance
/// of the type from a `SerialString` value.
#[macro_export]
macro_rules! try_from_serial_string {
    ($t:ty) => {
        impl TryFrom<$t> for $crate::serialise::SerialString {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(value: $t) -> Result<Self, $crate::serialise::SerialiseError> {
                Ok(value.get_serialised())
            }
        }
    };
}

/// Implements `TryFrom<T> for SerialString` for a type.
///
/// This macro generates an implementation that attempts to convert an instance
/// of the type into a `SerialString` value.
#[macro_export]
macro_rules! try_to_serial_string {
    ($t:ty) => {
        impl TryFrom<$crate::serialise::SerialString> for $t {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(
                value: $crate::serialise::SerialString,
            ) -> Result<Self, $crate::serialise::SerialiseError> {
                Ok(Self::new(value))
            }
        }
    };
}
