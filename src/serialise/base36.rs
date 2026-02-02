use crate::serialise::SerialString;

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Base36 {
    serialised: SerialString,
}

impl Base36 {
    pub fn new(serialised: SerialString) -> Self {
        Self { serialised }
    }

    pub fn get_serialised(self) -> SerialString {
        self.serialised
    }

    pub fn to_base36(bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "0".to_string();
        }

        if bytes.iter().all(|&b| b == 0) {
            return "0".to_string();
        }

        let mut n = bytes.to_vec();
        let mut out: Vec<u8> = Vec::new();

        while !n.is_empty() && n.iter().any(|&b| b != 0) {
            let mut rem: u32 = 0;
            for b in n.iter_mut() {
                let v = (rem << 8) | (*b as u32);
                *b = (v / 36) as u8;
                rem = v % 36;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        String::from_utf8(out).expect("base36 alphabet is valid utf8")
    }

    fn base36_to_bytes(base36: &str) -> Vec<u8> {
        let s = base36.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let c = c.to_ascii_lowercase();
            let digit = ALPHABET
                .iter()
                .position(|&b| b == c)
                .expect("invalid base36 character") as u32;

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = (*b as u32) * 36 + carry;
                *b = (v & 0xff) as u8;
                carry = v >> 8;
            }

            while carry > 0 {
                bytes.insert(0, (carry & 0xff) as u8);
                carry >>= 8;
            }
        }

        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }

        bytes
    }

    pub fn from_base36(base36: &str, size: usize) -> Vec<u8> {
        let mut bytes = Base36::base36_to_bytes(base36);

        if bytes.len() > size && size > 0 {
            panic!("base36 value does not fit in {} bytes", size);
        }

        if bytes.len() < size && size > 0 {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

#[macro_export]
macro_rules! impl_to_base36 {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Base36 {
            fn from(value: &$t) -> Self {
                let bytes = value.as_bytes();
                let string = $crate::serialise::Base36::to_base36(&bytes);
                $crate::serialise::Base36::new($crate::serialise::SerialString::new(
                    $crate::serialise::SerialiseType::Base36,
                    string,
                ))
            }
        }
    };
}

impl From<Base36> for SerialString {
    fn from(value: Base36) -> SerialString {
        value.get_serialised()
    }
}
