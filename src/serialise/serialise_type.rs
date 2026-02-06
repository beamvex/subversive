#[derive(Debug, Copy, Clone)]
pub enum SerialiseType {
    Base36,
    Base58,
    Base64,
    Uuid,
    Hex,
}
