#![no_std]

/// Hardware Abstraction Layer (HAL) interface
/// Defines traits that each platform must implement

/// Platform initialization trait
pub trait PlatformInit {
    /// Initialize all platform components
    fn init_platform(&mut self) -> Result<(), &'static str>;
    
    /// Setup memory management
    fn setup_memory(&mut self) -> Result<(), &'static str>;
    
    /// Setup interrupt handling
    fn setup_interrupts(&mut self) -> Result<(), &'static str>;
}

/// Interrupt controller trait
pub trait InterruptController {
    /// Register an interrupt handler
    fn register_handler(&mut self, vector: usize, handler: InterruptHandler) -> Result<(), &'static str>;
    
    /// Enable interrupts
    fn enable_interrupts(&mut self) -> Result<(), &'static str>;
    
    /// Disable interrupts
    fn disable_interrupts(&mut self) -> Result<(), &'static str>;
}

/// Interrupt handler function type
pub type InterruptHandler = fn() -> ();

/// Memory manager trait
pub trait MemoryManager {
    /// Allocate a page
    fn allocate_page(&mut self) -> Option<*mut u8>;
    
    /// Free a page
    fn free_page(&mut self, ptr: *mut u8) -> Result<(), &'static str>;
    
    /// Set page protection
    fn set_page_protection(&mut self, addr: usize, writable: bool, user: bool) -> Result<(), &'static str>;
}

/// Timer trait
pub trait Timer {
    /// Initialize timer
    fn init_timer(&mut self) -> Result<(), &'static str>;
    
    /// Get system ticks
    fn get_ticks(&self) -> u64;
    
    /// Set timer interrupt handler
    fn set_tick_handler(&mut self, handler: TimerHandler);
}

/// Timer handler function type
pub type TimerHandler = fn() -> ();

/// Console trait
pub trait Console {
    /// Write a character
    fn write_char(&mut self, c: u8);
    
    /// Write a string
    fn write_string(&mut self, s: &str);
    
    /// Flush output
    fn flush(&mut self);
}

/// CPU information trait
pub trait CPUInfo {
    /// Get number of CPU cores
    fn core_count(&self) -> u32;
    
    /// Detect CPU features
    fn detect_features(&mut self) -> Result<(), &'static str>;
    
    /// Get CPU identifier
    fn get_cpu_id(&self) -> u64;
}

/// Power state trait
pub trait PowerState {
    /// Halt CPU
    fn cpu_halt(&self) -> !;
    
    /// Sleep CPU
    fn cpu_sleep(&self) -> !;
    
    /// Reset system
    fn reset(&self) -> !;
}