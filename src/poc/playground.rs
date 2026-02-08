use std::fmt::Display;

#[derive(Debug, Default)]
struct Bytes(Vec<u8>);

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[test]
fn test_rust() {
    let string = Bytes(b"this is a test".to_vec());
    crate::debug!("{string:?}, {string}");
}
