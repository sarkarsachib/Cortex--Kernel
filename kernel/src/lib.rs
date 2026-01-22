#![no_std]

/// Cortex-μKernel Library
/// Provides kernel services and HAL interfaces

pub mod hal;

/// Kernel library modules
pub mod arch {
    pub mod x86_64 {
        pub mod boot;
        pub mod gdt;
        pub mod idt;
        pub mod memory;
        pub mod console;
        pub mod timer;
        pub mod cpuid;
        pub mod power;
        pub mod init;
    }
}

/// Re-export HAL types for external use
pub use hal::{
    PlatformInit, InterruptController, MemoryManager, 
    Timer, Console, CPUInfo, PowerState,
    InterruptHandler, TimerHandler,
};

/// Kernel initialization
pub fn kernel_init() -> Result<(), &'static str> {
    // Initialize platform
    crate::arch::x86_64::init::init_platform()?;
    
    // Additional kernel initialization can go here
    Ok(())
}

/// Kernel shutdown
pub fn kernel_shutdown() -> ! {
    // Shutdown using power management
    crate::arch::x86_64::power::shutdown();
}
