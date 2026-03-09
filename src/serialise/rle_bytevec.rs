/// Run-length encoded byte vector
/// can hold multiple byte vecs with a header indicating the length of each vec
pub struct RLEByteVec {
    data: Vec<u8>,
}

impl RLEByteVec {
    /// Create a new RLEByteVec
    #[must_use]
    pub const fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Get the data
    #[must_use]
    pub const fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}
