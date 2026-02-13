use crate::serialise::{Base36, SerialString, SerialiseError, SerialiseType, StructType};

/// Raw byte representation of serializable data.
///
/// This type represents the raw bytes of a serializable structure along with
/// its type information. It serves as an intermediate format between the
/// original data and its string representation.
#[derive(Debug, Clone)]
pub struct Bytes {
    /// The type of structure these bytes represent
    struct_type: StructType,
    /// The raw byte data
    bytes: Vec<u8>,
}

impl Bytes {
    /// Creates a new `Bytes` instance.
    ///
    /// # Arguments
    /// * `struct_type` - The type of structure these bytes represent
    /// * `bytes` - The raw byte data
    #[must_use = "This creates a new Bytes instance but does nothing if unused"]
    pub const fn new(struct_type: StructType, bytes: Vec<u8>) -> Self {
        Self { struct_type, bytes }
    }

    /// Returns the type of structure these bytes represent.
    #[must_use = "This returns the structure type but does nothing if unused"]
    pub const fn get_struct_type(&self) -> StructType {
        self.struct_type
    }

    /// Returns the raw byte data.
    #[must_use = "This returns the byte data but does nothing if unused"]
    pub fn get_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Attempts to convert these bytes into a string representation.
    ///
    /// # Arguments
    /// * `serialise_type` - The format to use for serialization
    ///
    /// # Returns
    /// The serialized string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The serialization format is not supported
    /// - The bytes cannot be encoded in the specified format
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_into_serialstring(
        self,
        serialise_type: SerialiseType,
    ) -> Result<SerialString, SerialiseError> {
        match serialise_type {
            SerialiseType::Base36 => self.try_into_serialstring_base36(),
            _ => Err(SerialiseError::new("Inavlid SerialiseType".to_string())),
        }
    }
    /// Attempts to convert these bytes into a base36-encoded string.
    ///
    /// # Returns
    /// The base36-encoded string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The bytes cannot be encoded in base36 format
    /// - The Base36 conversion fails
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_into_serialstring_base36(self) -> Result<SerialString, SerialiseError> {
        match Base36::try_from(self) {
            Ok(base36) => match base36.try_into() {
                Ok(serialstring) => Ok(serialstring),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }
}

/// Implements `TryFrom<Bytes>` for a type.
///
/// This macro generates an implementation that attempts to create an instance
/// of the type from a `Bytes` value. The implementation will:
/// 1. Extract the structure type and raw bytes
/// 2. Verify the structure type matches
/// 3. Convert the bytes into the target type
#[macro_export]
macro_rules! try_from_bytes {
    ($t:ty) => {
        /**
         * Create a new instance of $t from a serialised string.
         */
        impl TryFrom<$crate::serialise::Bytes> for $t {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(
                value: $crate::serialise::Bytes,
            ) -> Result<Self, $crate::serialise::SerialiseError> {
                let mut vec: Vec<u8> = vec![];
                vec.push(value.get_struct_type().try_into().unwrap());
                vec.extend_from_slice(&value.get_bytes());
                let vec: Result<$t, $crate::serialise::SerialiseError> = vec.try_into();
                if let Err(error) = vec {
                    return Err(error);
                }
                Ok(vec.unwrap())
            }
        }
    };
}

/// Implements `TryFrom<T> for Bytes` for a type.
///
/// This macro generates an implementation that attempts to convert an instance
/// of the type into a `Bytes` value. The implementation will:
/// 1. Convert the value into its byte representation
/// 2. Extract the structure type
/// 3. Create a new `Bytes` instance
#[macro_export]
macro_rules! try_to_bytes {
    ($t:ty) => {
        /**
         * Create a new instance of Bytes from a serialised string.
         */
        impl TryFrom<$t> for $crate::serialise::Bytes {
            type Error = $crate::serialise::SerialiseError;
            fn try_from(value: $t) -> Result<Self, $crate::serialise::SerialiseError> {
                // convert the string to bytes with the algorithm specified in the serialised string type
                let bytes: Result<Vec<u8>, $crate::serialise::SerialiseError> = value.try_into();
                if let Err(error) = bytes {
                    // error converting to bytes
                    return Err(error);
                }
                // bytes converted successfully
                let bytes = bytes.unwrap();
                // get the type code as the first byte
                let type_code: Result<
                    $crate::serialise::StructType,
                    $crate::serialise::SerialiseError,
                > = $crate::serialise::StructType::try_from(bytes[0]);
                if let Err(error) = type_code {
                    return Err(error);
                }
                // create the bytes type from the decoded bytes
                Ok(Self::new(type_code.unwrap(), bytes[1..].to_vec()))
            }
        }
    };
}
