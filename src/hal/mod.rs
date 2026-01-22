//! Hardware Abstraction Layer
//! 
//! The HAL provides a unified interface for hardware-specific operations
//! across all supported architectures.

pub use crate::hal::error::*;
pub use crate::hal::cpu::*;
pub use crate::hal::interrupt::*;
pub use crate::hal::memory::*;
pub use crate::hal::device::*;
pub use crate::hal::clock::*;

// Platform-specific HAL type
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::X86_64Hal;

#[cfg(target_arch = "aarch64")]
pub use crate::arch::arm64::Arm64Hal;

#[cfg(target_arch = "arm")]
pub use crate::arch::arm32::Arm32Hal;

#[cfg(target_arch = "riscv64")]
pub use crate::arch::riscv64::RiscV64Hal;

// Platform-specific initialization
pub fn create_hal() -> Box<dyn Hal> {
    #[cfg(target_arch = "x86_64")]
    {
        Box::new(X86_64Hal::new())
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        Box::new(Arm64Hal::new())
    }
    
    #[cfg(target_arch = "arm")]
    {
        Box::new(Arm32Hal::new())
    }
    
    #[cfg(target_arch = "riscv64")]
    {
        Box::new(RiscV64Hal::new())
    }
}

/// Main HAL trait - platform entry point
pub trait Hal {
    type Cpu: CpuHal;
    type Memory: MemoryHal;
    type Interrupt: InterruptHal;
    type Device: DeviceHal;
    type Clock: ClockHal;
    
    fn cpu(&self) -> &Self::Cpu;
    fn memory(&self) -> &Self::Memory;
    fn interrupt(&self) -> &Self::Interrupt;
    fn device(&self) -> &Self::Device;
    fn clock(&self) -> &Self::Clock;
    
    fn early_init(&mut self) -> HalResult<()>;
    fn late_init(&mut self) -> HalResult<()>;
}