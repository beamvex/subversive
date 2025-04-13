use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Represents the current shutdown state of the application
#[derive(Debug)]
pub struct ShutdownState {
    is_shutting_down: AtomicBool,
}

impl ShutdownState {
    /// Create a new shutdown state
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            is_shutting_down: AtomicBool::new(false),
        })
    }

    /// Check if shutdown has been initiated
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::SeqCst)
    }

    /// Initiate shutdown
    pub fn initiate_shutdown(&self) {
        self.is_shutting_down.store(true, Ordering::SeqCst);
    }
}
