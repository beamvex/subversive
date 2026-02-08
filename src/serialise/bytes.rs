use crate::serialise::StructType;

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
    pub const fn get_serialise_type(&self) -> StructType {
        self.struct_type
    }

    #[must_use]
    pub const fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}
