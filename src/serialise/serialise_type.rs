/// Supported serialization formats.
///
/// This enum represents the different formats that can be used to serialize
/// data structures into string representations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialiseType {
    /// Base36 encoding (0-9 and A-Z)
    Base36,
    /// Base58 encoding (Bitcoin-style, excluding similar-looking characters)
    Base58,
    /// Standard Base64 encoding
    Base64,
    /// U-U Encoding format
    Uuencode,
    /// Hexadecimal encoding (0-9 and A-F)
    Hex,
}
