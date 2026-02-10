pub mod algorithm;
pub mod as_bytes;
pub mod bytes;
pub mod from_bytes;
pub mod serial_string;
pub mod serialise_error;
pub mod serialise_type;
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

#[macro_export]
macro_rules! serialisable {
    ($t:ty) => {
        $crate::try_to_bytes!($t);
        $crate::try_from_bytes!($t);
    };
}

#[macro_export]
macro_rules! string_serialisable {
    ($t:ty) => {
        $crate::try_to_serial_string!($t);
        $crate::try_from_serial_string!($t);
    };
}
