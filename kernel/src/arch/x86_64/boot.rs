#![no_std]

/// Boot assembly functions
/// This module provides the interface to boot.asm assembly code

extern "C" {
    /// Main entry point called by bootloader
    pub fn _start() -> !;
}

/// Declare external assembly functions
extern "C" {
    pub fn kernel_main() -> !;
}