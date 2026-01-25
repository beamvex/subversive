use std::fmt::Display;

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Base36 {
    string: String,
}

impl Base36 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_base36_string(Self::to_base36(bytes))
    }

    pub fn from_base36_string(string: String) -> Self {
        Self { string }
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        Base36::base36_to_bytes(&self.string)
    }

    fn to_base36(bytes: &[u8]) -> String {
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

        if bytes.len() > size {
            panic!("base36 value does not fit in {} bytes", size);
        }

        if bytes.len() < size {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

impl Display for Base36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[macro_export]
macro_rules! serialise_base36 {
    ($t:ty) => {
        impl_from_base36!($t);
        impl_into_base36!($t);
    };
}

#[macro_export]
macro_rules! impl_from_base36 {
    ($t:ty) => {
        impl From<&$crate::serialise::Base36> for $t {
            fn from(value: &$crate::serialise::Base36) -> Self {
                let size: usize = std::mem::size_of::<Self>();
                let bytes = $crate::serialise::Base36::from_base36(&value.get_string(), size);
                <$t>::read_from(&bytes).unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_base36 {
    ($t:ty) => {
        impl From<&$t> for $crate::serialise::Base36 {
            fn from(value: &$t) -> Self {
                $crate::serialise::Base36::from_bytes(&value.as_bytes())
            }
        }
    };
}
