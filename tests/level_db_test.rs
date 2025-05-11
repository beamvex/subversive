#[cfg(test)]
mod tests {

    const MYDATABASE: &'static str = "db/mydatabase";
    const TOTAL_RECORDS: usize = 1_000_000;
    const LOG_INTERVAL: usize = 1_000;

    use rusty_leveldb::{LdbIterator, DB};
    use subversive_utils::test_utils::init_test_tracing;

    #[test]
    fn test_level_db() {
        init_test_tracing();
        let opt = rusty_leveldb::Options::default();

        let mut db = DB::open(MYDATABASE, opt).unwrap();

        for i in 0..TOTAL_RECORDS {
            let key = format!("key_{}", i).into_bytes();
            let value = format!("value_{}", i).into_bytes();
            db.put(&key, &value).unwrap();

            if (i + 1) % LOG_INTERVAL == 0 {
                tracing::info!("Inserted {} records", i + 1);
            }
        }

        // Verify a few random records
        for i in [0, 42, 999, 9999, TOTAL_RECORDS - 1] {
            let key = format!("key_{}", i).into_bytes();
            let value = format!("value_{}", i).into_bytes();
            assert_eq!(value, db.get(&key).unwrap().as_slice());
        }

        db.flush().unwrap();
    }

    #[test]
    fn test_level_db_read() {
        init_test_tracing();
        let opt = rusty_leveldb::Options::default();
        let mut db = DB::open(MYDATABASE, opt).unwrap();

        let mut iter = db.new_iter().unwrap();

        iter.seek(b"key_9900000");

        let mut i = 0;

        while iter.advance() {
            let mut key = Vec::new();
            let mut val = Vec::new();

            let _ok = iter.current(&mut key, &mut val);

            let key_str = String::from_utf8_lossy(&key);
            let val_str = String::from_utf8_lossy(&val);
            if (i + 1) % LOG_INTERVAL == 0 {
                tracing::info!("key: {}, value: {}", key_str, val_str);
            }
            i += 1;
        }
    }
}
