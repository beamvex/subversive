#[tokio::main]
pub async fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    main();
    assert!(true);
}
