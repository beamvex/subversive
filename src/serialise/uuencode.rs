use crate::serialise::{SerialString, SerialiseType};

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

#[macro_export]
macro_rules! impl_to_uuencode {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Uuencode {
            fn from(value: &$t) -> Self {
                let bytes = value.try_as_bytes().unwrap();
                let string = $crate::serialise::Uuencode::to_uuencode(&bytes);
                $crate::serialise::Uuencode::new($crate::serialise::SerialString::new(
                    $crate::serialise::SerialiseType::UUencode,
                    string,
                ))
            }
        }
    };
}

impl TryFrom<&Vec<u8>> for Uuencode {
    type Error = ();

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(SerialString::new(
            SerialiseType::UUencode,
            Self::to_uuencode(value),
        )))
    }
}

#[macro_export]
macro_rules! impl_from_uuencode {
    ($t:ty) => {
        impl From<$crate::serialise::Uuencode> for $t {
            fn from(value: $crate::serialise::Uuencode) -> Self {
                let serialised = value.get_serialised();
                let uu = serialised.get_string();
                let bytes = $crate::serialise::Uuencode::from_uuencode(&uu);
                <$t>::try_from_bytes(&bytes).unwrap()
            }
        }
    };
}

impl From<Uuencode> for SerialString {
    fn from(value: Uuencode) -> Self {
        value.get_serialised()
    }
}

impl From<SerialString> for Uuencode {
    fn from(value: SerialString) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_uuencode_roundtrip() {
        let test = b"this is a really good test";
        let uu = Uuencode::to_uuencode(test);
        crate::debug!("uuencode {uu}");
        let bytes = Uuencode::from_uuencode(&uu);
        assert_eq!(test, bytes.as_slice());
    }
}
