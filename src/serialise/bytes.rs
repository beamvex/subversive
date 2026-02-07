use crate::serialise::StructType;

#[derive(Debug)]
pub struct Bytes {
    struct_type: StructType,
    bytes: Vec<u8>,
}
