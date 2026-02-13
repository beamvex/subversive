/// Serialization algorithms and implementations.
pub mod algorithm;

/// Trait for types that can be converted to bytes.
pub mod as_bytes;

/// Raw byte representation of serializable data.
pub mod bytes;

/// Trait for types that can be constructed from bytes.
pub mod from_bytes;

/// String representation of serialized data.
pub mod serial_string;

/// Error type for serialization operations.
pub mod serialise_error;

/// Supported serialization formats.
pub mod serialise_type;

/// Types of serializable structures.
pub mod struct_type;

pub use algorithm::base36::Base36;
//pub use algorithm::base58::Base58;
//pub use algorithm::base64::Base64;
//pub use algorithm::hex::Hex;
//pub use algorithm::uuencode::Uuencode;
pub use as_bytes::AsBytes;
pub use bytes::Bytes;
pub use from_bytes::FromBytes;
pub use serial_string::SerialString;
pub use serialise_error::SerialiseError;
pub use serialise_type::SerialiseType;
pub use struct_type::StructType;

/// Implements serialization traits for a type.
///
/// This macro implements both `AsBytes` and `FromBytes` traits for a type,
/// allowing it to be converted to and from bytes.
#[macro_export]
macro_rules! serialisable {
    ($t:ty) => {
        $crate::try_to_bytes!($t);
        $crate::try_from_bytes!($t);
    };
}

/// Implements string serialization traits for a type.
///
/// This macro implements the traits needed to convert a type to and from
/// its string representation using various encoding formats.
#[macro_export]
macro_rules! string_serialisable {
    ($t:ty) => {
        $crate::try_to_serial_string!($t);
        $crate::try_from_serial_string!($t);
    };
}
