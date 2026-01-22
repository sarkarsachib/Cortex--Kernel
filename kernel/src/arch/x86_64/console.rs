#![no_std]

/// Console implementation for x86_64
/// Provides serial console output via 8250 UART

/// Serial port I/O addresses
const COM1_BASE: u16 = 0x3F8;

/// Console structure
pub struct Console {
    /// Base I/O port
    port: u16,
    /// Initialized flag
    initialized: bool,
}

impl Console {
    /// Create new console for specified COM port
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            initialized: false,
        }
    }

    /// Initialize the console
    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Ok(());
        }

        // Simple initialization - just mark as ready
        self.initialized = true;
        Ok(())
    }

    /// Write a single character
    pub fn write_char(&mut self, c: u8) {
        if !self.initialized {
            return;
        }

        // Simple implementation - just write to port
        self.outb(self.port, c);
    }

    /// Write a string
    pub fn write_string(&mut self, s: &str) {
        for &byte in s.as_bytes() {
            self.write_char(byte);
        }
    }

    /// Write string with newline
    pub fn write_line(&mut self, s: &str) {
        self.write_string(s);
        self.write_char(b'\n');
    }

    /// Flush output (ensure all data is sent)
    pub fn flush(&mut self) {
        // Nothing to do for serial
    }

    /// Output byte to port
    fn outb(&mut self, port: u16, value: u8) {
        // Simple implementation - for now just ignore the value
        let _ = (port, value);
        // In a real implementation, this would write to I/O port
    }
}

/// Global console instance
static mut CONSOLE: Console = Console::new(COM1_BASE);

/// Initialize the global console
pub fn init_console() -> Result<(), &'static str> {
    unsafe { CONSOLE.init() }
}

/// Write character to global console
pub fn putchar(c: u8) {
    unsafe { CONSOLE.write_char(c) }
}

/// Write string to global console
pub fn puts(s: &str) {
    unsafe { CONSOLE.write_string(s) }
}

/// Write string with newline to global console
pub fn println(s: &str) {
    unsafe { CONSOLE.write_line(s) }
}

/// Flush global console
pub fn flush() {
    unsafe { CONSOLE.flush() }
}