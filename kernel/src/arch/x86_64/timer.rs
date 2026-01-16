#![no_std]

/// Timer implementation for x86_64
/// Provides system timer using PIT (Programmable Interval Timer)

/// Global timer state
static mut TIMER_TICKS: u64 = 0;

/// Timer structure
pub struct Timer {
    /// System ticks
    ticks: u64,
}

impl Timer {
    /// Create new timer
    pub const fn new() -> Self {
        Self {
            ticks: 0,
        }
    }

    /// Initialize the timer
    pub fn init(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    /// Get system ticks
    pub fn get_ticks(&self) -> u64 {
        unsafe { TIMER_TICKS }
    }

    /// Set timer handler
    pub fn set_tick_handler(&mut self, _handler: fn()) {
        // Not implemented
    }

    /// Timer interrupt handler
    pub fn timer_interrupt(&mut self) {
        unsafe {
            TIMER_TICKS += 1;
        }
        self.ticks += 1;
    }

    /// Sleep for specified milliseconds
    pub fn sleep_ms(&self, ms: u32) {
        let start_ticks = self.get_ticks();
        let target_ticks = start_ticks + (ms as u64);

        while self.get_ticks() < target_ticks {
            core::hint::spin_loop();
        }
    }
}

/// Global timer instance
static mut TIMER: Timer = Timer::new();

/// Initialize the global timer
pub fn init_timer() -> Result<(), &'static str> {
    unsafe { TIMER.init() }
}

/// Get system ticks
pub fn get_ticks() -> u64 {
    unsafe { TIMER_TICKS }
}

/// Set timer handler
pub fn set_tick_handler(_handler: fn()) {
    unsafe { TIMER.set_tick_handler(_handler) }
}

/// Timer interrupt handler
pub fn timer_interrupt_handler() {
    unsafe { TIMER.timer_interrupt() }
}

/// Sleep for specified milliseconds
pub fn sleep_ms(ms: u32) {
    unsafe { TIMER.sleep_ms(ms) }
}