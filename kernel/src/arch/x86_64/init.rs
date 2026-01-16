#![no_std]

/// Platform initialization for x86_64
/// Initializes all hardware components and HAL services

/// Platform initialization result
pub type InitResult = Result<(), &'static str>;

/// Platform initialization structure
pub struct PlatformInit {
    /// Initialization status
    initialized: bool,
}

impl PlatformInit {
    /// Create new platform initializer
    pub const fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    /// Initialize all platform components
    pub fn init_platform(&mut self) -> InitResult {
        if self.initialized {
            return Ok(());
        }

        // Initialize components in order of dependency
        self.init_console()?;
        self.init_gdt()?;
        self.init_idt()?;
        self.init_memory()?;
        self.init_timer()?;
        self.init_cpu()?;
        self.init_power()?;

        self.initialized = true;
        Ok(())
    }

    /// Initialize console first for early output
    fn init_console(&mut self) -> InitResult {
        crate::arch::x86_64::console::init_console()?;
        Ok(())
    }

    /// Initialize Global Descriptor Table
    fn init_gdt(&mut self) -> InitResult {
        crate::arch::x86_64::gdt::init_gdt();
        Ok(())
    }

    /// Initialize Interrupt Descriptor Table
    fn init_idt(&mut self) -> InitResult {
        crate::arch::x86_64::idt::init_idt();
        Ok(())
    }

    /// Initialize memory management
    fn init_memory(&mut self) -> InitResult {
        crate::arch::x86_64::memory::init_memory()?;
        Ok(())
    }

    /// Initialize system timer
    fn init_timer(&mut self) -> InitResult {
        crate::arch::x86_64::timer::init_timer()?;
        Ok(())
    }

    /// Initialize CPU information detection
    fn init_cpu(&mut self) -> InitResult {
        crate::arch::x86_64::cpuid::init_cpu_info()?;
        Ok(())
    }

    /// Initialize power management
    fn init_power(&mut self) -> InitResult {
        crate::arch::x86_64::power::init_power()?;
        Ok(())
    }

    /// Setup memory (alias for init_memory)
    pub fn setup_memory(&mut self) -> InitResult {
        self.init_memory()
    }

    /// Setup interrupts (alias for init_idt)
    pub fn setup_interrupts(&mut self) -> InitResult {
        self.init_idt()
    }

    /// Check if platform is fully initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Global platform initializer
static mut PLATFORM_INIT: PlatformInit = PlatformInit::new();

/// Initialize the platform
pub fn init_platform() -> InitResult {
    unsafe { PLATFORM_INIT.init_platform() }
}

/// Setup memory management
pub fn setup_memory() -> InitResult {
    unsafe { PLATFORM_INIT.setup_memory() }
}

/// Setup interrupt handling
pub fn setup_interrupts() -> InitResult {
    unsafe { PLATFORM_INIT.setup_interrupts() }
}

/// Check if platform is initialized
pub fn is_initialized() -> bool {
    unsafe { PLATFORM_INIT.is_initialized() }
}