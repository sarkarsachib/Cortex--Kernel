//! Cortex-μKernel - Core library exports

pub mod arch;
pub mod hal;
pub mod kernel;
pub mod driver;
pub mod lib;

// Re-export core HAL types
pub use hal::{Hal, HalError, HalResult, create_hal};

// Re-export core kernel types
pub use kernel::{Scheduler, MemoryManager, IpcManager};