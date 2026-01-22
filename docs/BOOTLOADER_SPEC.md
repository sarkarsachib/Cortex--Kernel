# Cortex-μKernel Bootloader Interface Specification

## Overview

This document defines the bootloader interface requirements for the Cortex-μKernel across all supported architectures. The bootloader interface ensures consistent boot sequences and proper transfer of control to the kernel while maintaining architecture-specific requirements.

## Bootloader Protocols

### Multiboot2 (x86_64)

#### Multiboot2 Compliance

The Cortex-μKernel x86_64 implementation complies with the Multiboot2 specification for boot loader compatibility.

#### Boot Information Structure

```rust
#[repr(C)]
pub struct Multiboot2Info {
    pub total_size: u32,
    pub reserved: u32,
    // Tag entries follow
}

// Tag type constants
const MULTIBOOT2_TAG_TYPE_END: u32 = 0;
const MULTIBOOT2_TAG_TYPE_CMDLINE: u32 = 1;
const MULTIBOOT2_TAG_TYPE_BOOT_LOADER_NAME: u32 = 2;
const MULTIBOOT2_TAG_TYPE_MODULE: u32 = 3;
const MULTIBOOT2_TAG_TYPE_MEMORY_MAP: u32 = 6;
const MULTIBOOT2_TAG_TYPE_FRAMEBUFFER: u32 = 8;
const MULTIBOOT2_TAG_TYPE_ELF_SECTIONS: u32 = 9;
const MULTIBOOT2_TAG_TYPE_APM: u32 = 10;
const MULTIBOOT2_TAG_TYPE_EFI_MMAP: u32 = 13;
const MULTIBOOT2_TAG_TYPE_EFI_IHANDLER: u32 = 14;
const MULTIBOOT2_TAG_TYPE_EFI_SYSTEM_TABLE: u32 = 15;
```

#### Required Boot Tags

1. **Memory Map** (`MULTIBOOT2_TAG_TYPE_MEMORY_MAP`)
   - Physical memory layout
   - Available vs reserved regions
   - Memory type classification

2. **Boot Command Line** (`MULTIBOOT2_TAG_TYPE_CMDLINE`)
   - Kernel command line arguments
   - Boot options and parameters

3. **Boot Loader Name** (`MULTIBOOT2_TAG_TYPE_BOOT_LOADER_NAME`)
   - Identification of bootloader
   - Version information

#### Entry Point Requirements

```assembly
# x86_64 Multiboot2 entry point
.global _start
.section .boot

_start:
    # EBX register contains pointer to Multiboot2 information
    push %ebx           # Save boot info pointer
    push %eax           # Magic number (0x36D76289)
    
    call main_kernel_entry
    
    # Should never return
.loop:
    hlt
    jmp .loop
```

#### Expected Register State

- **EAX**: Multiboot2 magic number (0x36D76289)
- **EBX**: Physical address of Multiboot2 information structure
- **CS**: Code segment, flat 32-bit or 64-bit mode
- **DS, ES, FS, GS, SS**: Data segments, flat mode
- **ESP**: Valid stack pointer (typically 4MB from start of kernel)

### UEFI (ARM64)

#### UEFI Boot Services

The ARM64 implementation expects to be booted via UEFI firmware with boot services available.

#### System Table Structure

```rust
#[repr(C)]
pub struct EfiSystemTable {
    pub signature: u64,
    pub revision: u32,
    pub creator_id: u32,
    pub creator_major_version: u8,
    pub creator_minor_version: u8,
    pub firmware_revision: u8,
    pub padding: [u8; 5],
    pub console_in_handle: u64,
    pub con_in: *const EfiSimpleTextInputProtocol,
    pub console_out_handle: u64,
    pub con_out: *const EfiSimpleTextOutputProtocol,
    pub standard_error_handle: u64,
    pub std_err: *const EfiSimpleTextOutputProtocol,
    pub runtime_services: *const EfiRuntimeServices,
    pub boot_services: *const EfiBootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *const EfiConfigurationTable,
}
```

#### Required Boot Services

1. **Memory Services**
   - `AllocatePages()` for memory allocation
   - `FreePages()` for memory deallocation
   - `GetMemoryMap()` for memory layout

2. **Boot Time Services**
   - `ExitBootServices()` for transitioning to OS
   - `HandleProtocol()` for protocol access

3. **Runtime Services**
   - Time services for system time
   - Variable services for boot configuration

#### Entry Point Requirements

```rust
// ARM64 UEFI entry point
#[no_mangle]
pub extern "efiapi" fn efi_main(
    handle: *const EfiHandle,
    system_table: *const EfiSystemTable,
) -> usize {
    // System table pointer is first parameter
    // Convert to Rust references
    let st = unsafe { &*system_table };
    
    // Initialize UEFI boot services
    let boot_services = st.boot_services;
    
    // Exit boot services to transfer control to OS
    let (_memory_map_key, _map_size, _descriptor_size) = 
        exit_boot_services(handle, boot_services);
    
    // Call kernel main
    kernel_main();
    
    // Should never return
    loop {}
}
```

### Device Tree (ARM32, RISC-V64)

#### Flattened Device Tree (FDT)

Both ARM32 and RISC-V64 use Device Trees for hardware discovery and configuration.

#### Device Tree Structure

```rust
#[repr(C)]
pub struct FdtHeader {
    pub magic: u32,           // 0xD00DFEED
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}
```

#### Required Device Tree Nodes

1. **Root Node** (`/`)
   - Compatible string list
   - Model and manufacturer information
   - Boot CPU identification

2. **CPU Nodes** (`/cpus/cpu@0`)
   - CPU type and features
   - Clock frequency information
   - Interrupt controller connection

3. **Memory Node** (`/memory@0`)
   - Physical memory layout
   - Memory bank information
   - Base address and size

4. **Interrupt Controller** (`/interrupt-controller`)
   - Interrupt controller type
   - Interrupt specifier format
   - Interrupt parent relationship

5. **Timer Nodes** (`/timer`)
   - Timer frequency
   - Timer interrupt sources
   - Timer compatibility strings

#### Entry Point Requirements

```rust
// ARM32 Device Tree entry point
#[no_mangle]
pub extern "C" fn _start(fdt_ptr: usize) -> ! {
    // fdt_ptr is Device Tree blob address
    let fdt = unsafe { DeviceTree::from_ptr(fdt_ptr as *const u8) };
    
    // Parse Device Tree
    let memory_info = parse_memory_node(&fdt)?;
    let cpu_info = parse_cpu_node(&fdt)?;
    let interrupt_info = parse_interrupt_controller(&fdt)?;
    
    // Initialize kernel with Device Tree information
    kernel_init_with_fdt(fdt, memory_info, cpu_info, interrupt_info);
    
    loop {}
}

// RISC-V64 Device Tree entry point
#[no_mangle]
pub extern "C" fn _start(fdt_ptr: usize, hart_id: usize) -> ! {
    let fdt = unsafe { DeviceTree::from_ptr(fdt_ptr as *const u8) };
    
    // RISC-V passes hart ID as second parameter
    let current_hart = hart_id;
    
    // Boot sequence for this hart
    boot_hart(current_hart, fdt);
    
    loop {}
}
```

## Kernel Entry Requirements

### Register State

#### x86_64
- **RAX**: Multiboot2 magic number
- **RBX**: Boot information structure pointer
- **CS**: Code segment, flat mode
- **SS**: Stack segment
- **RSP**: Valid stack pointer
- **RFLAGS**: Interrupt flag cleared

#### ARM64
- **X0**: System table pointer (UEFI)
- **X1**: Boot services handle
- **X2**: Device Tree pointer (if applicable)
- **X3**: ACPI root pointer (if applicable)
- **PSTATE**: Exceptions masked, EL1

#### ARM32
- **R0**: Device Tree pointer (if applicable)
- **R1**: Machine type (legacy ARM)
- **R2**: ATAGS or Device Tree pointer
- **CPSR**: IRQ/FIQ disabled

#### RISC-V64
- **A0**: Device Tree pointer
- **A1**: Hart ID
- **A2**: FDT size
- **SSTATUS**: Interrupt disabled

### Stack Setup

#### Stack Requirements

Each architecture requires a valid stack setup by the bootloader:

```rust
// Example stack setup for x86_64
const STACK_SIZE: usize = 4096 * 4; // 16KB stack
static mut BOOT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let stack_top = unsafe { BOOT_STACK.as_ptr().add(STACK_SIZE) as usize };
    
    unsafe {
        core::arch::asm!(
            "mov rsp, {stack_top}",
            "push rbp",
            stack_top = in(reg) stack_top,
        );
    }
    
    kernel_main()
}
```

### Memory Management Unit (MMU) State

#### Page Table Requirements

Bootloader must set up minimal page tables for kernel execution:

#### x86_64 Page Tables
```rust
// Minimal page table for x86_64
const PAGE_TABLE_ENTRIES: usize = 512;

pub struct Pml4Entry(u64);
pub struct PdptEntry(u64);
pub struct PdEntry(u64);
pub struct PtEntry(u64);

impl Pml4Entry {
    pub const fn present() -> Self { Self(1 << 0) }
    pub const fn writable() -> Self { Self(1 << 1) }
    pub const fn user_accessible() -> Self { Self(1 << 2) }
    pub const fn executable() -> Self { Self(1 << 63) }
}

// Setup identity mapping for kernel
fn setup_identity_mapping() {
    // Map first 1GB with 2MB pages
    for i in 0..512 {
        let pdpt = get_pdpt();
        pdpt[i] = PtEntry::present() | PtEntry::writable() | (i << 21);
    }
}
```

#### ARM64 Page Tables
```rust
// ARM64 page table setup
fn setup_early_mmu() {
    // Configure TTBR0_EL1 (Translation Table Base Register 0)
    let ttbr0 = setup_level1_table();
    
    unsafe {
        core::arch::asm!(
            "msr ttbr0_el1, {ttbr0}",
            "tlbi vmalle1is",     // Invalidate all TLB entries
            "dsb ish",            // Data synchronization barrier
            "isb",                // Instruction synchronization barrier
            ttbr0 = in(reg) ttbr0,
        );
    }
    
    // Enable MMU
    unsafe {
        core::arch::asm!(
            "mrs {sctlr}, sctlr_el1",
            "orr {sctlr}, {sctlr}, #1",  // Set M bit (MMU enable)
            "msr sctlr_el1, {sctlr}",
            "isb",
            sctlr = out(reg) _,
        );
    }
}
```

## Memory Map Expectations

### x86_64 E820 Memory Map

```rust
#[repr(C)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub entry_type: u32,
    pub ext_attr: u32,
}

// Memory types
const E820_AVAILABLE: u32 = 1;
const E820_RESERVED: u32 = 2;
const E820_ACPI_RECLAIMABLE: u32 = 3;
const E820_ACPI_NVS: u32 = 4;
const E820_BAD_MEMORY: u32 = 5;
```

### ARM64/ARM32 Memory Layout

```rust
pub struct MemoryLayout {
    pub dram_base: usize,
    pub dram_size: usize,
    pub kernel_code_start: usize,
    pub kernel_code_end: usize,
    pub kernel_data_start: usize,
    pub kernel_data_end: usize,
    pub kernel_bss_start: usize,
    pub kernel_bss_end: usize,
    pub kernel_stack_top: usize,
}

// Typical ARM64 memory layout
pub const ARM64_MEMORY_LAYOUT: MemoryLayout = MemoryLayout {
    dram_base: 0x4000_0000,
    kernel_code_start: 0x4008_0000,
    kernel_code_end: 0x4008_0000 + 0x1000,
    kernel_data_start: 0x4010_0000,
    kernel_data_end: 0x4010_0000 + 0x1000,
    kernel_bss_start: 0x4020_0000,
    kernel_bss_end: 0x4020_0000 + 0x1000,
    kernel_stack_top: 0xFFFF_FF00_0000_0000,
};
```

### RISC-V64 Memory Layout

```rust
pub struct RiscVMemoryLayout {
    pub dram_base: usize,
    pub kernel_code_start: usize,
    pub kernel_code_end: usize,
    pub kernel_data_start: usize,
    pub kernel_data_end: usize,
    pub kernel_bss_start: usize,
    pub kernel_bss_end: usize,
    pub stack_top: usize,
    pub fdt_base: usize,
}

// Typical RISC-V64 memory layout
pub const RISCV64_MEMORY_LAYOUT: RiscVMemoryLayout = RiscVMemoryLayout {
    dram_base: 0x8000_0000,
    kernel_code_start: 0x8020_0000,
    kernel_code_end: 0x8020_1000,
    kernel_data_start: 0x8030_0000,
    kernel_data_end: 0x8030_2000,
    kernel_bss_start: 0x8040_0000,
    kernel_bss_end: 0x8040_1000,
    stack_top: 0xFFFF_FFFF_FFFF_FFFF,
    fdt_base: 0x8000_0000,
};
```

## Boot Protocol Variants

### Legacy BIOS Boot (x86_64)

For compatibility with older systems, legacy BIOS boot is supported:

```assembly
# Legacy BIOS boot entry point
.global _start_bios
.section .boot

_start_bios:
    # Setup initial segments
    cli
    xor %ax, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    mov %ax, %ss
    
    # Setup stack
    mov $0x90000, %esp
    
    # Call BIOS to get memory map
    int $0x15
    
    # Jump to protected mode
    lgdt gdtr
    
    # Enable protected mode
    mov %cr0, %eax
    or $1, %eax
    mov %eax, %cr0
    
    # Far jump to enter 32-bit mode
    ljmp $0x08, $protected_mode_entry

gdtr:
    .word gdt_end - gdt_start - 1
    .long gdt_start
```

### Unified Extensible Firmware Interface (UEFI)

UEFI provides modern boot services and firmware interface:

```rust
// UEFI Boot Services protocol
#[repr(C)]
pub struct EfiBootServices {
    pub table_header: EfiTableHeader,
    pub raise_tpl: extern "efiapi" fn(tpl: usize) -> usize,
    pub restore_tpl: extern "efiapi" fn(tpl: usize),
    pub allocate_pages: extern "efiapi" fn(
        ty: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
    ) -> *mut u8,
    pub free_pages: extern "efiapi" fn(
        memory: *mut u8,
        pages: usize,
    ) -> EfiStatus,
    // ... additional boot services
}
```

### Device Tree (FDT) Boot

Device Tree provides hardware description for ARM and RISC-V:

```rust
// FDT parsing functions
pub fn parse_device_tree(fdt: &DeviceTree) -> BootInfo {
    let memory_info = parse_memory_node(&fdt)
        .expect("Failed to parse memory node");
    
    let cpu_info = parse_cpu_node(&fdt)
        .expect("Failed to parse CPU node");
    
    let interrupt_info = parse_interrupt_controller(&fdt)
        .expect("Failed to parse interrupt controller");
    
    BootInfo {
        memory: memory_info,
        cpu: cpu_info,
        interrupt: interrupt_info,
        fdt: fdt.clone(),
    }
}
```

## Implementation Examples

### x86_64 Boot Implementation

```rust
pub struct X86_64BootInfo {
    pub multiboot_info: *const Multiboot2Info,
    pub memory_map: Vec<E820Entry>,
    pub framebuffer_info: Option<FramebufferInfo>,
}

pub fn boot() -> ! {
    // Parse Multiboot2 information
    let boot_info = unsafe {
        let mb_info = &*multiboot_info;
        parse_multiboot2_info(mb_info)
    };
    
    // Initialize HAL
    let mut hal = X86_64Hal::new();
    hal.early_init().expect("Failed to initialize HAL");
    
    // Initialize memory subsystem
    hal.memory().init_from_e820(&boot_info.memory_map)
        .expect("Failed to initialize memory");
    
    // Initialize interrupt subsystem
    hal.interrupt().init().expect("Failed to initialize interrupts");
    
    // Initialize CPU subsystem
    hal.cpu().init().expect("Failed to initialize CPU");
    
    // Initialize clock subsystem
    hal.clock().init().expect("Failed to initialize clock");
    
    // Transfer control to kernel
    kernel_main(hal);
    
    loop {}
}
```

### ARM64 Boot Implementation

```rust
pub struct Arm64BootInfo {
    pub system_table: *const EfiSystemTable,
    pub memory_map: EfiMemoryMap,
    pub device_tree: Option<DeviceTree>,
}

pub fn boot() -> ! {
    let system_table = get_system_table();
    
    // Initialize UEFI HAL
    let mut hal = Arm64Hal::new();
    hal.early_init().expect("Failed to initialize HAL");
    
    // Initialize memory from UEFI
    let memory_map = get_memory_map(system_table);
    hal.memory().init_from_uefi(&memory_map)
        .expect("Failed to initialize memory");
    
    // Initialize interrupt subsystem (GIC)
    hal.interrupt().init().expect("Failed to initialize interrupts");
    
    // Initialize CPU subsystem
    hal.cpu().init().expect("Failed to initialize CPU");
    
    // Exit boot services to transfer control to OS
    exit_boot_services();
    
    // Transfer control to kernel
    kernel_main(hal);
    
    loop {}
}
```

This bootloader interface specification provides the foundation for consistent boot sequences across all supported architectures while respecting each platform's specific requirements and conventions.