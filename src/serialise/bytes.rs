use crate::serialise::{Base36, SerialString, SerialiseError, SerialiseType, StructType};

#[derive(Debug)]
pub struct Bytes {
    struct_type: StructType,
    bytes: Vec<u8>,
}

impl Bytes {
    #[must_use]
    pub const fn new(struct_type: StructType, bytes: Vec<u8>) -> Self {
        Self { struct_type, bytes }
    }

    #[must_use]
    pub const fn get_struct_type(&self) -> StructType {
        self.struct_type
    }

    #[must_use]
    pub fn get_bytes(self) -> Vec<u8> {
        self.bytes
    }

    #[must_use]
    pub fn try_into_serialstring(
        self,
        serialise_type: SerialiseType,
    ) -> Result<SerialString, SerialiseError> {
        match serialise_type {
            SerialiseType::Base36 => self.try_into_serialstring_base36(),
            _ => Err(SerialiseError::new("Inavlid SerialiseType".to_string())),
        }
    }
    #[must_use]
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
