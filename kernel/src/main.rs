#![no_std]
#![no_main]

extern crate verifier;
extern crate slots;

/// x86_64-specific entry point
#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the kernel
    kernel_main()
}

/// Main kernel function
fn kernel_main() -> ! {
    // Initialize kernel library
    if let Err(e) = kernel::kernel_init() {
        // If initialization fails, print error and halt
        crate::arch::x86_64::console::println("Kernel init failed");
        crate::arch::x86_64::console::println(e);
        crate::arch::x86_64::power::cpu_halt();
    }

    // Create verifier and slot manager
    let verifier = verifier::Verifier::new();
    let slots = slots::SlotManager::new();

    // Verify kernel integrity
    verifier.verify_kernel();

    // Print startup message
    crate::arch::x86_64::console::println("Cortex-μKernel x86_64 ready");
    
    // Enable interrupts
    crate::arch::x86_64::idt::enable_interrupts();

    // Simple kernel loop - in a real kernel this would be the scheduler
    kernel_loop()
}

/// Main kernel loop
fn kernel_loop() -> ! {
    loop {
        // In a real kernel, this would be:
        // 1. Check for interrupts
        // 2. Run scheduled tasks
        // 3. Check for system events
        // 4. Power management
        
        // For now, just halt the CPU to save power
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Print panic message if console is available
    crate::arch::x86_64::console::println("PANIC: Unknown panic");
    
    // Halt the system
    crate::arch::x86_64::power::cpu_halt();
}

/// Unhandled exception handler
#[no_mangle]
pub extern "C" fn handle_exception() -> ! {
    crate::arch::x86_64::console::println("Unhandled exception!");
    crate::arch::x86_64::power::cpu_halt();
}