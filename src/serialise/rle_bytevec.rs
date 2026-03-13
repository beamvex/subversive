use std::rc::Rc;

use base_xx::{ByteVec, SerialiseError};

/// Run-length encoded byte vector
/// can hold multiple byte vecs with a header indicating the length of each vec
pub struct RLEByteVec {
    data: Vec<Rc<ByteVec>>,
}

fn encode_len(len: usize) -> Result<Vec<u8>, SerialiseError> {
    let len_u64 =
        u64::try_from(len).map_err(|_| SerialiseError::new("Length too large".to_string()))?;

    let needed_bytes = {
        let bits = 64u32.saturating_sub(len_u64.leading_zeros());
        let bytes = bits.div_ceil(8) as usize;
        if bytes == 0 {
            1
        } else {
            bytes
        }
    };

    if needed_bytes > 8 {
        return Err(SerialiseError::new("Length too large".to_string()));
    }

    let mut out = Vec::with_capacity(1 + needed_bytes);
    let prefix_bits = u8::try_from(needed_bytes - 1)
        .map_err(|_| SerialiseError::new("Length too large".to_string()))?;
    let prefix = prefix_bits << 5;
    out.push(prefix);
    out.extend_from_slice(&len_u64.to_le_bytes()[..needed_bytes]);
    Ok(out)
}

fn decode_len(bytes: &[u8], offset: usize) -> Result<(u64, usize), SerialiseError> {
    let prefix = *bytes
        .get(offset)
        .ok_or_else(|| SerialiseError::new("Unexpected end of input".to_string()))?;
    let len_bytes = ((prefix >> 5) as usize) + 1;
    let start = offset + 1;
    let end = start + len_bytes;

    let slice = bytes
        .get(start..end)
        .ok_or_else(|| SerialiseError::new("Unexpected end of input".to_string()))?;

    let mut buf = [0u8; 8];
    buf[..len_bytes].copy_from_slice(slice);
    Ok((u64::from_le_bytes(buf), 1 + len_bytes))
}

impl RLEByteVec {
    /// Create a new RLEByteVec
    #[must_use]
    #[allow(clippy::doc_markdown)]
    pub const fn new(data: Vec<Rc<ByteVec>>) -> Self {
        Self { data }
    }

    /// Get the data
    #[must_use]
    #[allow(clippy::doc_markdown)]
    pub const fn get_data(&self) -> &Vec<Rc<ByteVec>> {
        &self.data
    }

    /// Add data to the RLEByteVec
    #[allow(clippy::doc_markdown)]
    pub fn add_data(&mut self, data: Rc<ByteVec>) {
        self.data.push(data);
    }
}

impl Default for RLEByteVec {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl TryFrom<&RLEByteVec> for ByteVec {
    type Error = SerialiseError;

    fn try_from(value: &RLEByteVec) -> Result<Self, Self::Error> {
        let mut result: Vec<u8> = vec![];
        for data in value.get_data() {
            let bytes = data.get_bytes();
            let len = bytes.len();
            result.extend_from_slice(&encode_len(len)?);
            result.extend_from_slice(bytes);
        }
        Ok(Self::new(result))
    }
}

impl TryFrom<&ByteVec> for RLEByteVec {
    type Error = SerialiseError;

    fn try_from(value: &ByteVec) -> Result<Self, Self::Error> {
        let mut out: Vec<Rc<ByteVec>> = Vec::new();
        let bytes = value.get_bytes();

        let mut offset = 0usize;
        while offset < bytes.len() {
            let (len_u64, consumed) = decode_len(bytes, offset)?;
            offset += consumed;

            let len = usize::try_from(len_u64)
                .map_err(|_| SerialiseError::new("Length too large".to_string()))?;

            let end = offset
                .checked_add(len)
                .ok_or_else(|| SerialiseError::new("Length too large".to_string()))?;
            let slice = bytes
                .get(offset..end)
                .ok_or_else(|| SerialiseError::new("Unexpected end of input".to_string()))?;

            out.push(Rc::new(ByteVec::new(slice.to_vec())));
            offset = end;
        }

        Ok(Self::new(out))
    }
}

impl TryFrom<ByteVec> for RLEByteVec {
    type Error = SerialiseError;

    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_bytevec_roundtrip() {
        let mut rle = RLEByteVec::new(vec![]);
        rle.add_data(Rc::new(ByteVec::new(vec![1, 2, 3])));
        rle.add_data(Rc::new(ByteVec::new(vec![])));
        rle.add_data(Rc::new(ByteVec::new(vec![9; 300])));

        let encoded: ByteVec = match ByteVec::try_from(&rle) {
            Ok(encoded) => encoded,
            Err(e) => panic!("encode RLEByteVec: {e}"),
        };
        let decoded = match RLEByteVec::try_from(&encoded) {
            Ok(decoded) => decoded,
            Err(e) => panic!("decode RLEByteVec: {e}"),
        };

        assert_eq!(decoded.get_data().len(), 3);
        assert_eq!(decoded.get_data()[0].get_bytes(), &[1, 2, 3]);
        assert_eq!(decoded.get_data()[1].get_bytes(), &[]);
        assert_eq!(decoded.get_data()[2].get_bytes(), vec![9u8; 300].as_slice());
    }

    #[test]
    fn test_len_prefix_sizes() {
        let cases = [
            (0u64, 1usize),
            (0x12u64, 1usize),
            (0x1234u64, 2usize),
            (0x12_34_56u64, 3usize),
            (0x12_34_56_78u64, 4usize),
            (0x12_34_56_78_9Au64, 5usize),
            (0x12_34_56_78_9A_BCu64, 6usize),
            (0x12_34_56_78_9A_BC_DEu64, 7usize),
            (0x12_34_56_78_9A_BC_DE_F0u64, 8usize),
        ];

        for (len, expected_bytes) in cases {
            let Ok(len_usize) = usize::try_from(len) else {
                panic!("u64 length does not fit in usize");
            };
            let Ok(enc) = encode_len(len_usize) else {
                panic!("encode length failed");
            };
            assert_eq!(((enc[0] >> 5) as usize) + 1, expected_bytes);
            let Ok((dec, consumed)) = decode_len(&enc, 0) else {
                panic!("decode length failed");
            };
            assert_eq!(consumed, 1 + expected_bytes);
            assert_eq!(dec, len);
        }
    }
}
