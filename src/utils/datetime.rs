use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn to_last_time_block(t: SystemTime) -> SystemTime {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after unix epoch")
        .as_secs();

    let block_secs = (secs / 600) * 600;
    UNIX_EPOCH + Duration::from_secs(block_secs)
}

pub fn get_last_time_block() -> SystemTime {
    to_last_time_block(SystemTime::now())
}

#[test]
fn test_get_last_time_block() {
    let base = UNIX_EPOCH + Duration::from_secs(600 * 12345);
    let t = base + Duration::from_secs(5 * 60);
    let last_time_block = to_last_time_block(t);

    let iso = chrono::DateTime::<chrono::Utc>::from(last_time_block).to_rfc3339();
    println!("last_time_block: {}", iso);

    assert_eq!(last_time_block, base);
}