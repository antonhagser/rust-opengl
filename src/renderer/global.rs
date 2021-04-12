use std::{sync::RwLock, time::Instant};

/// Structure containing data needed for global use accross rendering pipeline.
/// <br>
/// Defines structure of ex. current VertexAttribArray index
pub struct Global {
    start_time: Instant,
}

impl Global {
    pub fn new() -> RwLock<Global> {
        let global = Global {
            start_time: Instant::now()
        };

        RwLock::new(global)
    }

    /// Get a reference to the global's start time.
    pub fn start_time(&self) -> &Instant {
        &self.start_time
    }
}