#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate verifier;
extern crate slots;

use core::panic::PanicInfo;

// Platform-specific entry points
#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    /// Performs a minimal x86_64 boot sequence and transfers control to `kernel_main`.
    ///
    /// This function performs the architecture-specific steps required to enter the kernel
    /// runtime on x86_64 and never returns.
    ///
    /// # Examples
    ///
    /// ```
    /// // Obtain the boot function as a value (do not call in tests; it does not return)
    /// let _boot_fn: fn() -> ! = kernel::x86_64::boot;
    /// ```
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
    /// Performs a minimal ARM64 boot sequence, disables interrupts, and transfers control to `kernel_main`.
    ///
    /// This function disables IRQs and all interrupt classes, calls `kernel_main`, and never returns. It is intended to be used as the early boot entry for AArch64 targets.
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
    /// Performs a minimal ARM32 (Thumb) boot sequence: disables IRQs and transfers control to `kernel_main`.
    ///
    /// This function never returns.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Platform entry point should call this; it disables interrupts and hands off to the kernel.
    /// boot();
    /// ```
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
    /// Performs minimal RISC-V CPU setup and transfers control to `kernel_main`.
    ///
    /// This function clears the machine status register and disables machine-level
    /// interrupt enables, then calls `kernel_main`. It never returns.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Called from the architecture-specific entry point; diverges and does not return.
    /// boot();
    /// ```
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

/// Kernel entry point invoked by the bootloader on x86_64.
///
/// This symbol is exported with C linkage and delegates control to the architecture-specific
/// boot sequence.
///
/// # Examples
///
/// ```rust,no_run
/// // Called by the bootloader on x86_64 targets.
/// // In normal builds this is provided as the program entry point and should not be invoked manually.
/// _start();
/// ```
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub extern "C" fn _start() -> ! {
    x86_64::boot()
}

/// Kernel entry symbol used on aarch64 that transfers execution to the architecture-specific boot sequence and never returns.
///
/// # Examples
///
/// ```no_run
/// // This symbol is used as the program entry; it is not called from normal Rust code.
/// ```
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn _start() -> ! {
    aarch64::boot()
}

/// ARM kernel entry point that transfers control to the architecture-specific boot routine.
///
/// # Examples
///
/// ```ignore
/// // Invoked by the platform bootstrap; calling this in user-space will not return.
/// _start();
/// ```
#[no_mangle]
#[cfg(target_arch = "arm")]
pub extern "C" fn _start() -> ! {
    arm::boot()
}

/// Kernel entry point for riscv64 that transfers control to the architecture boot routine.
///
/// The symbol `_start` is the first code executed by the bootloader and immediately hands
/// control to `riscv64::boot()`.
///
/// # Examples
///
/// ```no_run
/// // Confirm the `_start` symbol has the expected divergent signature.
/// extern "C" { fn _start() -> !; }
/// let _entry: extern "C" fn() -> ! = _start;
/// ```
#[no_mangle]
#[cfg(target_arch = "riscv64")]
pub extern "C" fn _start() -> ! {
    riscv64::boot()
}

/// Performs minimal kernel initialization, verifies kernel integrity, and halts the CPU.
///
/// This function constructs the kernel verifier and a slot manager, runs the kernel
/// integrity check, and then executes an architecture-specific halt or wait-for-interrupt
/// instruction to stop further execution.
///
/// # Examples
///
/// ```ignore
/// // This function halts the CPU; do not run in ordinary tests.
/// kernel_main();
/// ```
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

/// Halts the kernel by spinning indefinitely when a panic occurs.
///
/// This panic handler enters an infinite loop to stop further execution. The
/// provided `PanicInfo` is intentionally ignored.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}