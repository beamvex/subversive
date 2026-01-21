use zerocopy::AsBytes;

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

pub trait ToBase36 {
    fn to_base36(&self) -> String
    where
        Self: AsBytes,
    {
        let bytes = self.as_bytes();

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

pub trait FromBase36 {
    fn from_bytes(bytes: &[u8]) -> Self;

    fn from_base36(base36: &str) -> Self
    where
        Self: Sized,
    {
        let size: usize = std::mem::size_of::<Self>();

        let mut bytes = base36_to_bytes(base36);

        if bytes.len() > size {
            panic!("base36 value does not fit in {} bytes", size);
        }

        if bytes.len() < size {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return Self::from_bytes(&padded);
        }

        Self::from_bytes(&bytes)
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::utils::base36_to_bytes_64;

    #[test]
    fn test_base36_to_bytes_64() {
        let private_key_bytes = base36_to_bytes_64("z4mr3uhk64hsc8mzkhnh4d7w771s4z2vg8r46j828dnqs9spj7l41jxnmgz7fh4cb0h4qnui2ewhac76nzz525c1rq6mjmenwnj");

        let private_key = bytes_to_base36(&private_key_bytes);
        println!("private_key1: {}", private_key);

        assert_eq!(private_key, "z4mr3uhk64hsc8mzkhnh4d7w771s4z2vg8r46j828dnqs9spj7l41jxnmgz7fh4cb0h4qnui2ewhac76nzz525c1rq6mjmenwnj");
    }
}
    */
