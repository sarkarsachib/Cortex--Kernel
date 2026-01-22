//! Architecture-specific implementations

#![allow(unused)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub mod arm64;

#[cfg(target_arch = "arm")]
pub mod arm32;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

// Conditional exports
#[cfg(target_arch = "x86_64")]
pub use x86_64::X86_64Hal as CurrentHal;

#[cfg(target_arch = "aarch64")]
pub use arm64::Arm64Hal as CurrentHal;

#[cfg(target_arch = "arm")]
pub use arm32::Arm32Hal as CurrentHal;

#[cfg(target_arch = "riscv64")]
pub use riscv64::RiscV64Hal as CurrentHal;

// Architecture-specific types
#[cfg(target_arch = "x86_64")]
pub type CpuId = u32;

#[cfg(target_arch = "aarch64")]
pub type CpuId = u32;

#[cfg(target_arch = "arm")]
pub type CpuId = u32;

#[cfg(target_arch = "riscv64")]
pub type CpuId = u32;

// Feature detection macros
macro_rules! has_feature {
    ($feature:expr) => {
        #[cfg(target_feature = $feature)]
        true
    };
}