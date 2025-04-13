use std::sync::Arc;

/// Database context for the application
#[derive(Debug)]
pub struct DbContext {
    /// Path to the database file
    pub path: String,
}

impl DbContext {
    /// Create a new database context
    pub fn new(path: String) -> Arc<Self> {
        Arc::new(Self { path })
    }
}
