#![no_std]

/// Power state management for x86_64
/// Provides CPU power control and system reset functionality

/// Power management states
#[derive(Clone, Copy)]
pub enum PowerState {
    /// CPU halt (lowest power)
    Halt,
    /// CPU sleep (slight power saving)
    Sleep,
    /// Full system reset
    Reset,
    /// Shutdown (ACPI)
    Shutdown,
}

/// Power management structure
pub struct PowerManager {
    /// Current power state
    current_state: PowerState,
}

impl PowerManager {
    /// Create new power manager
    pub const fn new() -> Self {
        Self {
            current_state: PowerState::Halt,
        }
    }

    /// Initialize power management
    pub fn init(&mut self) -> Result<(), &'static str> {
        // Initialize power management subsystems
        self.current_state = PowerState::Halt;
        Ok(())
    }

    /// Halt the CPU
    pub fn cpu_halt(&self) -> ! {
        unsafe {
            // Execute HLT instruction in a loop
            loop {
                core::arch::asm!("hlt");
            }
        }
    }

    /// CPU sleep (similar to halt)
    pub fn cpu_sleep(&self) -> ! {
        self.cpu_halt()
    }

    /// Reset the system
    pub fn reset(&self) -> ! {
        unsafe {
            // Triple fault by loading invalid IDT
            let idt_ptr: u64 = 0;
            core::arch::asm!(
                "lidt [{0}]",
                in(reg) &idt_ptr as *const u64,
                options(nostack)
            );
            
            // Trigger interrupt to cause triple fault
            core::arch::asm!("int $0");
            
            // If we somehow get here, halt
            self.cpu_halt();
        }
    }

    /// Shutdown the system
    pub fn shutdown(&self) -> ! {
        self.reset()
    }

    /// Set power state
    pub fn set_power_state(&mut self, state: PowerState) {
        self.current_state = state;
    }

    /// Get current power state
    pub fn current_power_state(&self) -> PowerState {
        self.current_state
    }
}

/// Global power manager instance
static mut POWER_MANAGER: PowerManager = PowerManager::new();

/// Initialize the global power manager
pub fn init_power() -> Result<(), &'static str> {
    unsafe { POWER_MANAGER.init() }
}

/// Halt the CPU
pub fn cpu_halt() -> ! {
    unsafe { POWER_MANAGER.cpu_halt() }
}

/// CPU sleep
pub fn cpu_sleep() -> ! {
    unsafe { POWER_MANAGER.cpu_sleep() }
}

/// Reset the system
pub fn reset() -> ! {
    unsafe { POWER_MANAGER.reset() }
}

/// Shutdown the system
pub fn shutdown() -> ! {
    unsafe { POWER_MANAGER.shutdown() }
}

/// Set power state
pub fn set_power_state(state: PowerState) {
    unsafe { POWER_MANAGER.set_power_state(state) }
}

/// Get current power state
pub fn current_power_state() -> PowerState {
    unsafe { POWER_MANAGER.current_power_state() }
}