#[derive(Debug)]
struct Bytes(Vec<u8>);

#[test]
fn test_rust() {
    let string = Bytes(b"this is a test".to_vec());
    crate::debug!("{string:?}");
}
