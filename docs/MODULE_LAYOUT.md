# Cortex-μKernel Module Layout Documentation

## Overview

This document defines the complete module layout and directory structure for the Cortex-μKernel project, implementing the Hardware Abstraction Layer (HAL) architecture and providing clear separation between architecture-specific and architecture-independent code.

## Complete Directory Structure

```
cortex-mu-kernel/
├── docs/                          # Project documentation
│   ├── KERNEL_SPEC.md              # Kernel specification document
│   ├── HAL_ARCHITECTURE.md         # HAL architecture documentation
│   ├── MODULE_LAYOUT.md            # This document
│   ├── BOOTLOADER_SPEC.md          # Bootloader interface specification
│   ├── TEST_STRATEGY.md            # Testing and validation strategy
│   └── ARCHITECTURE.md             # Master index document
├── src/                           # Source code
│   ├── lib.rs                      # Core library exports
│   ├── main.rs                     # Binary entry point (legacy)
│   ├── arch/                       # Architecture-specific code
│   │   ├── mod.rs                  # Architecture module exports
│   │   ├── x86_64/
│   │   │   ├── mod.rs              # x86_64 module exports
│   │   │   ├── boot.rs             # Multiboot2 bootloader interface
│   │   │   ├── cpu.rs              # CPUID detection, MSR operations
│   │   │   ├── interrupt.rs        # APIC, IDT, exception handling
│   │   │   ├── memory.rs           # Page tables, E820 memory map
│   │   │   ├── io.rs               # Port I/O, MMIO, MSR access
│   │   │   └── gdt.rs              # Global Descriptor Table setup
│   │   ├── arm64/
│   │   │   ├── mod.rs              # ARM64 module exports
│   │   │   ├── boot.rs             # UEFI bootloader interface
│   │   │   ├── cpu.rs              # MIDR detection, system registers
│   │   │   ├── interrupt.rs        # GICv2/v3 management
│   │   │   ├── memory.rs           # Page tables, MMU setup
│   │   │   └── io.rs               # MMIO access, system registers
│   │   ├── arm32/
│   │   │   ├── mod.rs              # ARM32 module exports
│   │   │   ├── boot.rs             # Device tree bootloader interface
│   │   │   ├── cpu.rs              # CP15 registers, MPU setup
│   │   │   ├── interrupt.rs        # NVIC management
│   │   │   ├── memory.rs           # Page tables, MPU configuration
│   │   │   └── io.rs               # MMIO access, peripheral registers
│   │   └── riscv64/
│   │       ├── mod.rs              # RISC-V64 module exports
│   │       ├── boot.rs             # Device tree bootloader interface
│   │       ├── cpu.rs              # CSR access, extension detection
│   │       ├── interrupt.rs        # CLIC/PLIC management
│   │       ├── memory.rs           # Page tables, PMP configuration
│   │       └── io.rs               # MMIO access, control registers
│   ├── hal/                        # Hardware Abstraction Layer
│   │   ├── mod.rs                  # HAL module exports and core traits
│   │   ├── cpu.rs                  # CPU interface traits and types
│   │   ├── interrupt.rs            # Interrupt controller traits
│   │   ├── memory.rs               # Memory management traits
│   │   ├── device.rs               # Device bus and driver traits
│   │   ├── clock.rs                # Timer/clock interface traits
│   │   └── error.rs                # HAL error types and handling
│   ├── kernel/                     # Architecture-independent kernel
│   │   ├── mod.rs                  # Kernel module exports
│   │   ├── scheduler.rs            # Task scheduling algorithms
│   │   ├── syscall.rs              # System call interface
│   │   ├── ipc.rs                  # Inter-process communication
│   │   ├── sync.rs                 # Synchronization primitives
│   │   ├── memory.rs               # Virtual memory management
│   │   ├── process.rs              # Process and task management
│   │   └── init.rs                 # Kernel initialization
│   ├── driver/                     # Device drivers
│   │   ├── mod.rs                  # Driver module exports
│   │   ├── serial.rs               # UART/serial driver
│   │   ├── timer.rs                # System timer driver
│   │   ├── gpio.rs                 # General Purpose I/O driver
│   │   ├── rtc.rs                 # Real-Time Clock driver
│   │   └── network.rs              # Network interface driver
│   └── lib/                        # Utility libraries
│       ├── mod.rs                  # Library module exports
│       ├── bitflags.rs             # Bit flag utilities
│       ├── logging.rs              # Kernel logging framework
│       ├── allocator.rs             # Memory allocator
│       ├── spinlock.rs             # Spin lock primitives
│       ├── atomic.rs               # Atomic operations
│       ├── list.rs                 # Linked list implementations
│       ├── bitmap.rs               # Bitmap data structures
│       └── macros.rs               # Kernel macros
├── build/                         # Build system
│   ├── build.sh                   # Main build orchestrator
│   ├── size_check.sh              # Binary size validation
│   ├── test.sh                    # Test suite runner
│   ├── qemu_x86_64.sh            # QEMU boot for x86_64
│   ├── qemu_arm64.sh              # QEMU boot for ARM64
│   ├── qemu_arm32.sh              # QEMU boot for ARM32
│   ├── qemu_riscv64.sh            # QEMU boot for RISC-V64
│   ├── x86_64.ld                 # x86_64 linker script
│   ├── arm64.ld                   # ARM64 linker script
│   ├── arm32.ld                   # ARM32 linker script
│   └── riscv64.ld                # RISC-V64 linker script
├── .cargo/                       # Cargo configuration
│   ├── config.toml                # Cargo build configuration
│   ├── x86_64-cortex-kernel.json # x86_64 target specification
│   ├── aarch64-cortex-kernel.json# ARM64 target specification
│   ├── armv7-cortex-kernel.json  # ARM32 target specification
│   └── riscv64-cortex-kernel.json# RISC-V64 target specification
├── kernel/                      # Kernel crate
│   ├── Cargo.toml                # Kernel crate configuration
│   └── src/                     # Kernel source (legacy structure)
│       ├── main.rs              # Binary entry point
│       └── lib.rs              # Kernel library
├── verifier/                   # Verification library
│   ├── Cargo.toml              # Verifier crate configuration
│   └── src/
│       └── lib.rs             # Verification library
├── slots/                     # Slot management library
│   ├── Cargo.toml             # Slots crate configuration
│   └── src/
│       └── lib.rs            # Slot management library
└── .github/workflows/         # CI/CD configuration
    └── build.yml             # GitHub Actions workflow
```

## Module Descriptions

### Core HAL Module (`src/hal/`)

The Hardware Abstraction Layer provides architecture-independent interfaces for hardware operations.

#### `src/hal/mod.rs`
- **Purpose**: Core HAL exports and platform initialization
- **Contents**:
  - `Hal` trait definition
  - Platform HAL implementation exports
  - Early boot sequence coordination
  - Error handling integration

#### `src/hal/cpu.rs`
- **Purpose**: CPU interface and capability detection
- **Contents**:
  - `CpuHal` trait definition
  - CPU feature detection interfaces
  - CPU control operations
  - Cache management operations

#### `src/hal/interrupt.rs`
- **Purpose**: Interrupt controller abstraction
- **Contents**:
  - `InterruptHal` trait definition
  - IRQ management interfaces
  - Priority and routing management
  - Interrupt controller types

#### `src/hal/memory.rs`
- **Purpose**: Memory management abstraction
- **Contents**:
  - `MemoryHal` trait definition
  - Page table interfaces
  - Address translation operations
  - Memory discovery and mapping

#### `src/hal/device.rs`
- **Purpose**: Device enumeration and management
- **Contents**:
  - `DeviceHal` trait definition
  - Bus scanning interfaces
  - Device capability reporting
  - Resource management

#### `src/hal/clock.rs`
- **Purpose**: Clock and timer management
- **Contents**:
  - `ClockHal` trait definition
  - System timer interfaces
  - High-resolution timer support
  - Power management integration

#### `src/hal/error.rs`
- **Purpose**: HAL error handling
- **Contents**:
  - `HalError` enum definition
  - Error recovery mechanisms
  - Error reporting and logging
  - Recovery strategies

### Architecture-Specific Modules (`src/arch/`)

Each architecture implements the HAL traits and provides platform-specific functionality.

#### x86_64 Architecture (`src/arch/x86_64/`)

**`src/arch/x86_64/mod.rs`**
- x86_64 platform HAL implementation
- CPU-specific type definitions
- Platform initialization entry point
- Integration with kernel scheduling

**`src/arch/x86_64/boot.rs`**
- Multiboot2 compliance
- Boot information parsing
- Early boot sequence
- Memory map processing

**`src/arch/x86_64/cpu.rs`**
- CPUID instruction handling
- MSR (Model Specific Register) access
- Feature detection and reporting
- Cache and TLB management

**`src/arch/x86_64/interrupt.rs`**
- Local APIC management
- I/O APIC handling
- Interrupt Descriptor Table (IDT)
- Exception handling setup

**`src/arch/x86_64/memory.rs`**
- 4-level page table implementation
- E820 memory map processing
- Physical memory management
- TLB shootdown handling

**`src/arch/x86_64/io.rs`**
- Port I/O operations
- Memory-mapped I/O
- MSR read/write operations
- Hardware register access

**`src/arch/x86_64/gdt.rs`**
- Global Descriptor Table setup
- Segment descriptor configuration
- TSS (Task State Segment) management
- Protection ring setup

#### ARM64 Architecture (`src/arch/arm64/`)

**`src/arch/arm64/mod.rs`**
- ARM64 platform HAL implementation
- AArch64-specific type definitions
- Platform initialization entry point
- Exception level management

**`src/arch/arm64/boot.rs`**
- UEFI interface compliance
- System table processing
- Early boot sequence
- Device tree loading

**`src/arch/arm64/cpu.rs`**
- MIDR register access
- System register operations
- ARM64 feature detection
- Cache maintenance operations

**`src/arch/arm64/interrupt.rs`**
- GICv2/v3 distributor management
- GIC CPU interface
- SPI/PPI/SGI interrupt handling
- Priority and affinity management

**`src/arch/arm64/memory.rs`**
- 4-level page table implementation
- MMU configuration and control
- Memory attribute management
- Cache coherency operations

**`src/arch/arm64/io.rs`**
- Memory-mapped I/O access
- System register operations
- Control register access
- Hardware peripheral interfaces

#### ARM32 Architecture (`src/arch/arm32/`)

**`src/arch/arm32/mod.rs`**
- ARM32 platform HAL implementation
- ARMv7-M specific type definitions
- Platform initialization entry point
- Exception mode management

**`src/arch/arm32/boot.rs`**
- Device tree processing
- Boot information parsing
- Early boot sequence
- Memory region discovery

**`src/arch/arm32/cpu.rs`**
- CP15 coprocessor access
- MPU (Memory Protection Unit) setup
- ARM32 feature detection
- Cache and TLB management

**`src/arch/arm32/interrupt.rs`**
- NVIC (Nested Vectored Interrupt Controller)
- Interrupt priority management
- Exception vector table
- Interrupt enable/disable

**`src/arch/arm32/memory.rs`**
- 2-level page table implementation
- MPU region configuration
- Memory attribute setup
- Access permission management

**`src/arch/arm32/io.rs`**
- Memory-mapped I/O access
- Peripheral register access
- GPIO configuration
- Hardware control interfaces

#### RISC-V64 Architecture (`src/arch/riscv64/`)

**`src/arch/riscv64/mod.rs`**
- RISC-V64 platform HAL implementation
- RV64-specific type definitions
- Platform initialization entry point
- Privilege level management

**`src/arch/riscv64/boot.rs`**
- Device tree processing
- Boot information parsing
- Early boot sequence
- Hart (CPU core) discovery

**`src/arch/riscv64/cpu.rs`**
- CSR (Control and Status Register) access
- RISC-V extension detection
- Feature capability reporting
- Cache management operations

**`src/arch/riscv64/interrupt.rs`**
- CLIC (Core Local Interrupt Controller)
- PLIC (Platform Level Interrupt Controller)
- Interrupt priority and routing
- Exception handling setup

**`src/arch/riscv64/memory.rs`**
- Sv39/Sv48 page table implementation
- PMP (Physical Memory Protection)
- Memory attribute configuration
- Address translation

**`src/arch/riscv64/io.rs`**
- Memory-mapped I/O access
- Control and status register access
- Peripheral interfaces
- Hardware control operations

### Kernel Module (`src/kernel/`)

Architecture-independent kernel functionality.

**`src/kernel/mod.rs`**
- Kernel module exports
- Initialization coordination
- Core kernel interfaces

**`src/kernel/scheduler.rs`**
- Task scheduling algorithms
- Priority management
- Context switching
- Load balancing

**`src/kernel/syscall.rs`**
- System call interface
- ABI compatibility
- Parameter validation
- Service routing

**`src/kernel/ipc.rs`**
- Inter-process communication
- Message passing
- Shared memory
- Synchronization

**`src/kernel/sync.rs`**
- Synchronization primitives
- Mutexes, semaphores
- Condition variables
- Spin locks

**`src/kernel/memory.rs`**
- Virtual memory management
- Memory allocation
- Page fault handling
- Memory protection

**`src/kernel/process.rs`**
- Process and task management
- Process creation/destruction
- Task state management
- Resource allocation

**`src/kernel/init.rs`**
- Kernel initialization sequence
- Subsystem initialization
- Service registration
- Boot completion

### Device Drivers (`src/driver/`)

Hardware device driver implementations.

**`src/driver/mod.rs`**
- Driver module exports
- Driver registration
- Device management

**`src/driver/serial.rs`**
- UART/serial port driver
- Interrupt-driven I/O
- Buffer management
- Protocol handling

**`src/driver/timer.rs`**
- System timer driver
- High-resolution timers
- Timer interrupt handling
- Scheduling integration

**`src/driver/gpio.rs`**
- General Purpose I/O driver
- Pin configuration
- Interrupt generation
- Hardware control

**`src/driver/rtc.rs`**
- Real-Time Clock driver
- Time reading/writing
- Alarm functionality
- Time synchronization

**`src/driver/network.rs`**
- Network interface driver
- Packet transmission/reception
- Protocol support
- Buffer management

### Utility Libraries (`src/lib/`)

Common utilities and data structures.

**`src/lib/mod.rs`**
- Library module exports
- Common type definitions
- Utility function exports

**`src/lib/bitflags.rs`**
- Bit flag operations
- Flag manipulation utilities
- Bit testing and setting

**`src/lib/logging.rs`**
- Kernel logging framework
- Log levels and filtering
- Output interfaces
- Performance logging

**`src/lib/allocator.rs`**
- Memory allocator implementations
- Buddy allocator
- Slab allocator
- Memory tracking

**`src/lib/spinlock.rs`**
- Spin lock primitives
- Lock implementations
- Deadlock detection
- Performance optimization

**`src/lib/atomic.rs`**
- Atomic operation wrappers
- Lock-free data structures
- Memory ordering
- Architecture-specific implementations

**`src/lib/list.rs`**
- Linked list implementations
- Singly/doubly linked lists
- Lock-free variants
- Intrusive lists

**`src/lib/bitmap.rs`**
- Bitmap data structures
- Bit manipulation
- Memory-efficient operations
- Allocation tracking

**`src/lib/macros.rs`**
- Kernel macro definitions
- Code generation utilities
- Debug macros
- Performance macros

## Implementation Guidelines

### Module Dependencies

```
main.rs
    ↓
arch/, hal/, kernel/
    ↓
driver/, lib/
```

### Compilation Structure

1. **Core Library** (`src/lib.rs`)
   - Exports core types and utilities
   - Provides public interfaces
   - Manages module dependencies

2. **Binary Entry** (`src/main.rs`)
   - Platform-specific entry points
   - Boot sequence coordination
   - HAL initialization

3. **Conditional Compilation**
   - Platform-specific code via `#[cfg(target_arch = "...")]`
   - Feature-gated functionality
   - Optional component inclusion

### Error Handling Strategy

1. **HAL Errors**: Platform-specific error handling
2. **Kernel Errors**: Architecture-independent error management
3. **Driver Errors**: Device-specific error reporting
4. **Recovery Mechanisms**: Graceful degradation and recovery

### Testing Structure

1. **Unit Tests**: Individual module testing
2. **Integration Tests**: Cross-module functionality
3. **Platform Tests**: Architecture-specific validation
4. **Boot Tests**: System initialization verification

This module layout provides a solid foundation for implementing the Cortex-μKernel with clear separation of concerns, maintainable code structure, and efficient compilation.