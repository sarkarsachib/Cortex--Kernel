#![no_std]

/// x86_64 HAL (Hardware Abstraction Layer) modules
/// Exports all platform-specific functionality

pub mod boot;
pub mod gdt;
pub mod idt;
pub mod memory;
pub mod console;
pub mod timer;
pub mod cpuid;
pub mod power;
pub mod init;

/// Re-export commonly used types and functions
pub use boot::_start;
pub use init::{
    init_platform, setup_memory, setup_interrupts, 
    is_initialized, PlatformInit, InitResult
};
pub use memory::{memory_manager, init_memory, MemoryManager};
pub use console::{init_console, putchar, puts, println, flush, Console};
pub use timer::{init_timer, get_ticks, set_tick_handler, timer_interrupt_handler, sleep_ms, Timer};
pub use gdt::{init_gdt, get_gdt_ptr, get_tss};
pub use idt::{init_idt, enable_interrupts, disable_interrupts, register_handler};
pub use cpuid::{init_cpu_info, cpu_info, get_cpu_id, CpuInfo};
pub use power::{init_power, cpu_halt, cpu_sleep, reset, shutdown, set_power_state, current_power_state, PowerManager, PowerState};