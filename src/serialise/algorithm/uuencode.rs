use crate::{
    serialisable,
    serialise::{SerialString, SerialiseError, SerialiseType},
    string_serialisable,
};

#[derive(Debug)]
pub struct Uuencode {
    serialised: SerialString,
}

impl Uuencode {
    #[must_use]
    pub const fn new(serialised: SerialString) -> Self {
        Self { serialised }
    }

    #[must_use]
    pub fn get_serialised(self) -> SerialString {
        self.serialised
    }

    const fn enc6(v: u8) -> u8 {
        let v = v & 0x3f;
        if v == 0 {
            b'`'
        } else {
            v + 0x20
        }
    }

    const fn dec6(c: u8) -> Option<u8> {
        match c {
            b'`' | b' ' => Some(0),
            0x20..=0x5f => Some((c - 0x20) & 0x3f),
            _ => None,
        }
    }

    fn enc_len(n: usize) -> u8 {
        Self::enc6(u8::try_from(n).expect("uuencode chunk length must fit in u8"))
    }

    fn dec_len(c: u8) -> Option<usize> {
        Self::dec6(c).map(usize::from)
    }

    // Uuencode bytes using the traditional uuencode line format (45 bytes per line).
    //
    // Output has one or more lines. Each line begins with an encoded length character,
    // followed by encoded data, and ends with `\n`. The final line is "`\n".
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_uuencode(bytes: &[u8]) -> String {
        let mut out: Vec<u8> = Vec::new();

        for chunk in bytes.chunks(45) {
            out.push(Self::enc_len(chunk.len()));

            for triple in chunk.chunks(3) {
                let b0 = triple[0];
                let b1 = *triple.get(1).unwrap_or(&0);
                let b2 = *triple.get(2).unwrap_or(&0);

                let c0 = (b0 >> 2) & 0x3f;
                let c1 = ((b0 << 4) | (b1 >> 4)) & 0x3f;
                let c2 = ((b1 << 2) | (b2 >> 6)) & 0x3f;
                let c3 = b2 & 0x3f;

                out.push(Self::enc6(c0));
                out.push(Self::enc6(c1));
                out.push(Self::enc6(c2));
                out.push(Self::enc6(c3));
            }

            out.push(b'\n');
        }

        out.push(b'`');
        out.push(b'\n');

        String::from_utf8(out).expect("uuencode output must be valid UTF-8")
    }

    /// Decode a uuencoded string (traditional uuencode line format) into bytes.
    ///
    /// # Panics
    ///
    /// Panics if `uuencoded` contains invalid uuencode characters or malformed lines.
    #[must_use]
    pub fn from_uuencode(uuencoded: &str) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        for line in uuencoded.lines() {
            if line.is_empty() {
                continue;
            }

            let mut it = line.as_bytes().iter().copied();
            let len_ch = it
                .next()
                .expect("uuencode line must have a length character");
            let line_len = Self::dec_len(len_ch).expect("invalid uuencode length character");
            if line_len == 0 {
                break;
            }

            let mut produced = 0usize;
            while produced < line_len {
                let a = it.next().expect("truncated uuencode data");
                let b = it.next().expect("truncated uuencode data");
                let c = it.next().expect("truncated uuencode data");
                let d = it.next().expect("truncated uuencode data");

                let a = Self::dec6(a).expect("invalid uuencode character");
                let b = Self::dec6(b).expect("invalid uuencode character");
                let c = Self::dec6(c).expect("invalid uuencode character");
                let d = Self::dec6(d).expect("invalid uuencode character");

                let o0 = (a << 2) | (b >> 4);
                let o1 = (b << 4) | (c >> 2);
                let o2 = (c << 6) | d;

                for o in [o0, o1, o2] {
                    if produced < line_len {
                        out.push(o);
                        produced += 1;
                    }
                }
            }
        }

        out
    }
}

impl TryFrom<Uuencode> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Uuencode) -> Result<Self, Self::Error> {
        Ok(Uuencode::from_uuencode(value.get_serialised().get_string()))
    }
}

impl TryFrom<Vec<u8>> for Uuencode {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::UUencode,
            Self::to_uuencode(&value),
        )))
    }
}

serialisable!(Uuencode);
string_serialisable!(Uuencode);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::serialise::Bytes;
    use crate::serialise::SerialString;
    use crate::serialise::SerialiseError;
    use crate::serialise::StructType;

    #[test]
    pub fn test_uuencode() {
        let test = b"this is a really good test";
        let test_bytes = Bytes::new(StructType::HASH, test.to_vec());
        let uuencode: Uuencode = test_bytes.try_into().unwrap();
        crate::debug!("uuencode {uuencode:?}");
        let serialised: SerialString = uuencode.try_into().unwrap();
        let serialised_str = serialised.get_string();
        crate::debug!("test_bytes_restored {serialised_str}");
        let deserialised: Uuencode = serialised.try_into().unwrap();
        let test_bytes_restored: Bytes = deserialised.try_into().unwrap();
        assert_eq!(test, test_bytes_restored.get_bytes().as_slice());
    }

    #[test]
    pub fn test_invalid_uuencode() {
        let test = b"this is a failure test; its a little bit manufactured as this shouldnt be possible via code";
        let test_bytes = test.to_vec();
        let mut badvec: Vec<u8> = vec![];
        badvec.push(99);
        badvec.extend_from_slice(&test_bytes);

        let uuencode: Uuencode = Uuencode::new(SerialString::new(
            SerialiseType::UUencode,
            Uuencode::to_uuencode(&badvec),
        ));
        crate::debug!("uuencode {uuencode:?}");

        let serialised: SerialString = uuencode.try_into().unwrap();

        let deserialised: Uuencode = serialised.try_into().unwrap();
        let test_bytes_restored: Result<Bytes, SerialiseError> = deserialised.try_into();

        assert!(test_bytes_restored.is_err());
    }
}
