pub fn bytes_to_base36(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

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
