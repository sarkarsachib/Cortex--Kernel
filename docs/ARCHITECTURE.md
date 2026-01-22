# Cortex-μKernel Architecture Documentation

## Overview

Welcome to the comprehensive architecture documentation for Cortex-μKernel, a minimal multi-architecture microkernel supporting x86_64, ARM64, ARM32, and RISC-V platforms.

## Documentation Index

This documentation suite provides complete architectural guidance for understanding, implementing, and extending the Cortex-μKernel system.

### Core Specification Documents

#### [Kernel Specification](KERNEL_SPEC.md)
- **Executive Summary**: High-level overview of kernel goals and target platforms
- **Core Principles**: Design philosophy, performance targets, security model
- **Supported Architectures**: x86_64, ARM64, ARM32, RISC-V capability matrix
- **System Interfaces**: Syscall ABI, exception handling, interrupt model
- **Memory Management**: Virtual memory, paging, TLB, protection schemes
- **Process/Task Model**: Task scheduling, context switching, synchronization
- **IPC & Synchronization**: Mutexes, semaphores, condition variables, message passing
- **Device I/O Model**: Interrupt handling, DMA, device drivers, bus abstraction
- **Module System**: Kernel modules, loading/unloading, symbol resolution

#### [HAL Architecture](HAL_ARCHITECTURE.md)
- **HAL Layer Responsibilities**: CPU detection, memory mapping, interrupt routing
- **Platform-Specific Modules**: Directory structure for each architecture
- **Architecture Traits**: Defined interfaces each platform must implement
- **Boot Sequence**: Bootloader → HAL init → Kernel init for each platform
- **CPU Capabilities**: CPUID/MIDR detection, feature flags, capability reporting
- **Interrupt Handling**: APIC/GIC initialization, exception vector setup
- **Memory Initialization**: Early page table setup, memory discovery
- **Device Detection**: Bus scanning (PCI, device tree), resource assignment
- **Platform Initialization Priority**: Order of operations during boot

#### [Module Layout](MODULE_LAYOUT.md)
- **Directory Structure**: Complete source tree organization
- **Architecture-Specific Code**: Platform implementation details
- **HAL Implementation**: Hardware abstraction layer structure
- **Kernel Core**: Architecture-independent functionality
- **Device Drivers**: Hardware device driver architecture
- **Utility Libraries**: Common utilities and data structures
- **Compilation Structure**: Build organization and dependencies
- **Implementation Guidelines**: Coding standards and patterns

#### [Bootloader Specification](BOOTLOADER_SPEC.md)
- **Multiboot2 Compliance**: For x86_64 (tags, memory map, module loading)
- **UEFI Expectations**: For ARM64 (entry point, device tree loading)
- **Device Tree (FDT)**: For ARM32 and RISC-V (expected structure)
- **Kernel Entry Requirements**: Stack setup, register state, MMU state
- **Boot Protocol Variants**: Legacy BIOS boot vs. UEFI, device tree vs. ACPI
- **Memory Map Expectations**: Platform-specific memory layout
- **Implementation Examples**: Boot sequence code for each architecture

#### [Testing Strategy](TEST_STRATEGY.md)
- **Unit Tests**: HAL trait implementations testing
- **Integration Tests**: Cross-platform boot validation (QEMU)
- **Hardware Capability Detection**: CPUID, MIDR parsing tests
- **Performance Validation**: Timing constraints and benchmarks
- **Security Testing**: Memory protection and isolation
- **Test Infrastructure**: Automated testing framework

## Architecture Summary

### Design Philosophy

Cortex-μKernel follows a **minimalist microkernel architecture** with these core principles:

1. **Hardware Abstraction**: Clean separation via HAL layer
2. **Multi-Architecture**: Single codebase supporting 4 platforms
3. **Size Optimization**: Strict 20KB binary size limit
4. **Deterministic Performance**: Real-time capable with bounded latency
5. **Memory Safety**: Leveraging Rust's safety guarantees

### Supported Platforms

| Architecture | Boot Protocol | Primary Use Case |
|-------------|---------------|-----------------|
| **x86_64** | Multiboot2 | Desktop/Server, High Performance |
| **ARM64** | UEFI | Mobile/Embedded, Energy Efficient |
| **ARM32** | Device Tree | IoT/Microcontrollers, Real-time |
| **RISC-V64** | Device Tree | Research/Academic, Open Hardware |

### Key Components

#### Hardware Abstraction Layer (HAL)
```
┌─────────────────────────────────────────────────────────────┐
│                    HAL Interface                            │
├─────────────────┬─────────────────┬─────────────────────────┤
│      CPU        │    Memory       │     Interrupt           │
│   Interface     │   Management    │     Controller          │
├─────────────────┼─────────────────┼─────────────────────────┤
│     Device      │     Clock       │        Error            │
│   Management    │   Management    │       Handling          │
└─────────────────┴─────────────────┴─────────────────────────┘
        │                  │                    │
┌──────▼──────┐    ┌───────▼───────┐    ┌───────▼───────┐
│  x86_64     │    │    ARM64      │    │   RISC-V64    │
│  Specific   │    │    Specific   │    │   Specific    │
└─────────────┘    └───────────────┘    └───────────────┘
        │
┌───────▼───────┐
│    ARM32       │
│   Specific     │
└───────────────┘
```

#### Kernel Core Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     Kernel Core                             │
├─────────────────┬─────────────────┬─────────────────────────┤
│   Scheduler     │   Memory Mgr    │      IPC Manager       │
│                 │                 │                        │
├─────────────────┼─────────────────┼─────────────────────────┤
│ Synchronization │  Process Mgmt   │     Time Manager       │
│   Primitives    │                 │                        │
├─────────────────┴─────────────────┴─────────────────────────┤
│                  System Call Interface                       │
└─────────────────────────────────────────────────────────────┘
        │                          │                    │
┌───────▼──────┐    ┌─────────────▼──────┐    ┌───────▼──────┐
│   User Apps   │    │   Device Drivers    │    │   HAL Layer  │
│               │    │                    │    │              │
└──────────────┘    └────────────────────┘    └──────────────┘
```

### Memory Layout

#### Address Space Organization
```
User Space (0x0000_0000 - 0x7FFF_FFFF)
├── Code Section
├── Data Section  
├── Heap (growing up)
└── Stack (growing down)

Kernel Space (0x8000_0000 - 0xFFFF_FFFF)
├── Kernel Code
├── Kernel Data
├── Page Tables
└── Kernel Heap
```

#### Platform-Specific Memory Layouts

**x86_64**:
- Base: 0x100000 (1MB)
- Page Tables: 4-level hierarchy
- Virtual Address Space: 48-bit

**ARM64**:
- Base: 0x40000000 (1GB)
- Page Tables: 4-level hierarchy
- Virtual Address Space: 48-bit

**ARM32**:
- Base: 0x00000000
- Page Tables: 2-level hierarchy
- Virtual Address Space: 32-bit

**RISC-V64**:
- Base: 0x80200000 (2GB)
- Page Tables: Sv39/Sv48
- Virtual Address Space: 39/48-bit

### Boot Sequence Flow

```
Bootloader Entry
    ↓
HAL Early Init
    ↓
Memory Discovery & Setup
    ↓
CPU Detection & Init
    ↓
Interrupt Controller Init
    ↓
Timer/Clock Init
    ↓
Device Enumeration
    ↓
HAL Late Init
    ↓
Kernel Main
```

### Performance Targets

| Metric | Target | Platform Notes |
|--------|--------|----------------|
| **Boot Time** | < 100ms | All platforms |
| **Interrupt Latency** | < 10μs | Architecture dependent |
| **Context Switch** | < 1μs | Same priority tasks |
| **Memory Overhead** | < 4KB | Kernel data structures |
| **Binary Size** | < 20KB | Strict requirement |

### Security Model

1. **Memory Protection**: Hardware-enforced page tables per task
2. **Capability-Based Access**: Object capabilities for resource access
3. **Privilege Separation**: User/kernel space separation
4. **Minimal Attack Surface**: Reduced syscall surface, no dynamic loading
5. **Memory Safety**: Rust's ownership system in kernel context

## Implementation Status

### ✅ Completed
- [x] Project structure and build system
- [x] Target specifications for all 4 architectures
- [x] Linker scripts for each platform
- [x] Basic boot entry points
- [x] Documentation suite
- [x] HAL trait definitions
- [x] Architecture-specific stubs

### 🚧 In Progress
- [ ] HAL trait implementations
- [ ] Architecture-specific code completion
- [ ] Device driver implementations
- [ ] Full kernel functionality
- [ ] Comprehensive test suite

### 📋 Planned
- [ ] Performance optimization
- [ ] Additional device drivers
- [ ] Advanced security features
- [ ] Documentation refinements

## Quick Start

### Building the Kernel

```bash
# Build all targets
./build/build.sh all

# Build specific target
./build/build.sh x86_64

# Run tests
./build/test.sh

# Check binary sizes
./build/size_check.sh

# Boot in QEMU (x86_64)
./build/qemu_x86_64.sh
```

### Development Workflow

1. **Implement HAL Traits**: Add platform-specific implementations
2. **Write Device Drivers**: Implement drivers using HAL interfaces
3. **Add Kernel Functionality**: Extend architecture-independent kernel
4. **Test Integration**: Validate on all platforms via QEMU
5. **Performance Validation**: Ensure size and timing constraints

## Contributing

### Code Organization

- **HAL Layer**: Platform-agnostic interfaces in `src/hal/`
- **Architecture Code**: Platform-specific in `src/arch/{platform}/`
- **Kernel Core**: Generic kernel in `src/kernel/`
- **Drivers**: Device-specific in `src/driver/`
- **Utilities**: Common code in `src/lib/`

### Guidelines

1. **Minimal Implementation**: Follow the < 20KB constraint
2. **Performance First**: Prioritize deterministic performance
3. **Safety Critical**: Leverage Rust's safety guarantees
4. **Testing Required**: Add tests for all new functionality
5. **Documentation**: Update docs for all changes

## API Reference

### Core HAL Types

```rust
// Main HAL trait
pub trait Hal {
    type Cpu: CpuHal;
    type Memory: MemoryHal;
    type Interrupt: InterruptHal;
    type Device: DeviceHal;
    type Clock: ClockHal;
    
    fn early_init(&mut self) -> HalResult<()>;
    fn late_init(&mut self) -> HalResult<()>;
}

// Platform creation
pub fn create_hal() -> Box<dyn Hal>;

// Architecture selection
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::X86_64Hal;
```

### Error Handling

```rust
// HAL error types
#[derive(Debug, Clone, Copy)]
pub enum HalError {
    UnsupportedFeature,
    InitializationFailed,
    ResourceUnavailable,
    InvalidParameter,
    Timeout,
    PermissionDenied,
    DeviceNotFound,
}

// Result type
pub type HalResult<T> = Result<T, HalError>;
```

## Support and Resources

### Documentation Files

- **[KERNEL_SPEC.md](KERNEL_SPEC.md)**: Complete kernel specification
- **[HAL_ARCHITECTURE.md](HAL_ARCHITECTURE.md)**: HAL design and interfaces
- **[MODULE_LAYOUT.md](MODULE_LAYOUT.md)**: Source code organization
- **[BOOTLOADER_SPEC.md](BOOTLOADER_SPEC.md)**: Boot protocol specifications
- **[TEST_STRATEGY.md](TEST_STRATEGY.md)**: Testing and validation approach

### Build System

- **[BUILD.md](../BUILD.md)**: Build system documentation
- **[STRUCTURE.md](../STRUCTURE.md)**: Project structure overview
- **[rust-toolchain.toml](../rust-toolchain.toml)**: Rust toolchain configuration
- **[Cargo.toml](../Cargo.toml)**: Workspace configuration

### External Resources

- **Multiboot2 Specification**: https://www.gnu.org/software/grub/manual/multiboot2/
- **UEFI Specification**: https://uefi.org/specifications
- **Device Tree Specification**: https://devicetree.org/
- **RISC-V Specifications**: https://riscv.org/technical/specifications/

## Version Information

- **Current Version**: 0.1.0
- **Documentation Version**: Architecture v1.0
- **Supported Architectures**: x86_64, ARM64, ARM32, RISC-V64
- **Minimum Rust Version**: nightly-2023-xx-xx

---

*This architecture documentation serves as the definitive guide for understanding, implementing, and extending the Cortex-μKernel project. For implementation details, refer to the specific specification documents listed above.*