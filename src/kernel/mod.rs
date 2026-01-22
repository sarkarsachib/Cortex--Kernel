//! Kernel module exports

// Core kernel components (stub implementations)
pub struct Scheduler;
pub struct MemoryManager;
pub struct IpcManager;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryManager {
    pub fn new() -> Self {
        Self
    }
}

impl IpcManager {
    pub fn new() -> Self {
        Self
    }
}