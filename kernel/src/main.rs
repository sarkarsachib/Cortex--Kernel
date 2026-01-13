#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate verifier;
extern crate slots;

use core::panic::PanicInfo;

// Platform-specific entry points
#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub fn boot() -> ! {
        unsafe {
            // Minimal x86_64 boot sequence
            core::arch::asm!(
                "cli",           // Disable interrupts
                "lgdt [{0}]",    // Load GDT (placeholder)
                in(reg) 0usize,
            );
        }

        // Simple kernel entry point
        kernel_main();

        loop {}
    }
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    pub fn boot() -> ! {
        unsafe {
            // Minimal ARM64 boot sequence
            core::arch::asm!(
                "mrs x0, daif",  // Get interrupt flags
                "msr daifset, #2", // Disable IRQs
                "msr daifset, #7", // Disable all interrupts
            );
        }

        kernel_main();

        loop {}
    }
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub fn boot() -> ! {
        unsafe {
            // Minimal ARM32 boot sequence (Thumb mode)
            core::arch::asm!(
                "cpsid i",  // Disable IRQs
            );
        }

        kernel_main();

        loop {}
    }
}

#[cfg(target_arch = "riscv64")]
pub mod riscv64 {
    pub fn boot() -> ! {
        unsafe {
            // Minimal RISC-V boot sequence
            core::arch::asm!(
                "csrw mstatus, {0}",  // Clear mstatus
                "csrw mie, {0}",       // Disable interrupt enables
                in(reg) 0u64,
            );
        }

        kernel_main();

        loop {}
    }
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub extern "C" fn _start() -> ! {
    x86_64::boot()
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn _start() -> ! {
    aarch64::boot()
}

#[no_mangle]
#[cfg(target_arch = "arm")]
pub extern "C" fn _start() -> ! {
    arm::boot()
}

#[no_mangle]
#[cfg(target_arch = "riscv64")]
pub extern "C" fn _start() -> ! {
    riscv64::boot()
}

fn kernel_main() {
    // Minimal kernel initialization
    let verifier = verifier::Verifier::new();
    let _slots = slots::SlotManager::new();

    // Verify kernel integrity
    verifier.verify_kernel();

    // Simple halt for now
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("hlt");

        #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
        core::arch::asm!("wfi");

        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("wfi");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
