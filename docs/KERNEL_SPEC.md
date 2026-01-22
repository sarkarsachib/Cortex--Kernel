# Cortex-μKernel Specification

## Executive Summary

Cortex-μKernel is a minimal, multi-architecture microkernel designed for embedded and real-time systems. The kernel provides essential services including task scheduling, memory management, inter-process communication, and device abstraction across four target architectures: x86_64, ARM64, ARM32, and RISC-V 64-bit.

### Core Design Goals

- **Size Optimization**: Target binary size < 20KB per platform
- **Deterministic Performance**: Real-time capable with bounded interrupt latency
- **Multi-Architecture**: Single codebase supporting x86_64, ARM64, ARM32, and RISC-V
- **Hardware Abstraction**: Clean HAL layer enabling portability
- **Security**: Memory protection, capability-based security model
- **Minimal Dependencies**: Pure Rust implementation with no external runtime

## Core Principles

### Design Philosophy

1. **Minimalist Architecture**: Microkernel design with essential services only
2. **Hardware Abstraction**: Clean separation between architecture-specific and generic code
3. **Deterministic Execution**: Predictable timing for real-time applications
4. **Memory Safety**: Leverage Rust's safety guarantees while maintaining kernel privileges
5. **Modularity**: Well-defined interfaces enabling component substitution

### Performance Targets

- **Boot Time**: < 100ms on all platforms
- **Interrupt Latency**: < 10μs (platform dependent)
- **Context Switch**: < 1μs for same-priority tasks
- **Memory Overhead**: < 4KB for kernel data structures
- **Code Size**: < 20KB per platform (strict requirement)

### Security Model

- **Memory Protection**: Hardware-enforced page tables per task
- **Capability-Based Access**: Object capabilities for resource access
- **Privilege Separation**: User/kernel space separation
- **Secure Boot**: Verified boot chain with cryptographic verification
- **Attack Surface Reduction**: Minimal syscall surface, no dynamic code loading

## Supported Architectures

### Architecture Capability Matrix

| Feature | x86_64 | ARM64 | ARM32 | RISC-V64 |
|---------|--------|-------|-------|----------|
| Virtual Memory | ✓ | ✓ | ✓ | ✓ |
| Hardware Interrupts | ✓ | ✓ | ✓ | ✓ |
| Timer Support | ✓ | ✓ | ✓ | ✓ |
| Debug Support | ✓ | ✓ | ✓ | ✓ |
| Cache Management | ✓ | ✓ | ✓ | ✓ |
| Atomic Operations | ✓ | ✓ | ✓ | ✓ |
| SIMD Support | ✗ | ✗ | ✗ | ✗ |

### Platform-Specific Details

#### x86_64
- **CPU Requirements**: 64-bit mode, PAE support
- **Memory Model**: 4-level page tables, 48-bit virtual addresses
- **Interrupt Model**: APIC-based, 256 vector space
- **Boot Protocol**: Multiboot2 compliance
- **Performance**: High performance with branch prediction

#### ARM64 (AArch64)
- **CPU Requirements**: ARMv8-A architecture
- **Memory Model**: 4-level page tables, 48-bit virtual addresses
- **Interrupt Model**: GICv2/v3 support
- **Boot Protocol**: UEFI compliance
- **Performance**: Energy-efficient with big.LITTLE support

#### ARM32 (ARMv7-M)
- **CPU Requirements**: ARMv7-M architecture, Cortex-M4+
- **Memory Model**: 2-level page tables, 32-bit virtual addresses
- **Interrupt Model**: NVIC with configurable priority
- **Boot Protocol**: Device Tree or ACPI
- **Performance**: Real-time optimized with deterministic interrupt handling

#### RISC-V64
- **CPU Requirements**: RV64GC architecture
- **Memory Model**: Sv39/Sv48 page tables, 39/48-bit virtual addresses
- **Interrupt Model**: CLIC/PLIC support
- **Boot Protocol**: Device Tree
- **Performance**: RISC-V optimized instruction pipeline

## System Interfaces

### Syscall ABI

#### Core System Calls

```rust
// Process management
fn sys_process_create(entry: fn(), stack_size: usize) -> Result<ProcessId>;
fn sys_process_exit(exit_code: i32) -> !;
fn sys_process_wait(pid: ProcessId) -> Result<i32>;

// Memory management
fn sys_memory_alloc(size: usize) -> Result<*mut u8>;
fn sys_memory_free(ptr: *mut u8, size: usize) -> Result<()>;
fn sys_memory_map(addr: *mut u8, size: usize, prot: Protection) -> Result<()>;

// IPC
fn sys_ipc_send(dst: ProcessId, message: &[u8]) -> Result<()>;
fn sys_ipc_receive() -> Result<(ProcessId, Vec<u8>)>;
fn sys_ipc_reply(dst: ProcessId, message: &[u8]) -> Result<()>;

// Synchronization
fn sys_mutex_create() -> Result<MutexId>;
fn sys_mutex_acquire(id: MutexId, timeout: u64) -> Result<()>;
fn sys_mutex_release(id: MutexId) -> Result<()>;
```

#### Syscall Convention

- **Calling Convention**: Platform-specific ABI (System V x86_64, AAPCS64 ARM64, etc.)
- **Error Handling**: Negative return values, extended error codes via errno
- **Parameter Passing**: Registers first, stack for overflow
- **Return Values**: Result<T, KernelError> style with detailed error codes

### Exception Handling

#### Exception Types

1. **Hardware Exceptions**
   - Divide by zero, overflow, invalid opcode
   - Page faults, general protection faults
   - Alignment checks, machine checks

2. **Software Exceptions**
   - System calls (architecture-specific entry)
   - Breakpoints, single stepping
   - Software interrupts for IPC

#### Exception Handling Flow

```
Exception Occurs
    ↓
Save CPU State
    ↓
Determine Exception Type
    ↓
Route to Handler
    ↓
Handle Exception
    ↓
Restore State (if resumable)
    ↓
Return to Context
```

### Interrupt Model

#### Interrupt Sources

1. **Timer Interrupts**
   - System timer for scheduling
   - High-resolution timers for applications
   - Watchdog timers for system health

2. **Device Interrupts**
   - Serial/UART communication
   - GPIO and external devices
   - DMA completion notifications

3. **Software Interrupts**
   - Inter-processor interrupts
   - System call notifications
   - Debug and tracing interrupts

#### Interrupt Handling

- **Nested Interrupts**: Supported with priority-based nesting
- **Interrupt Masking**: Per-IRQ and global masking capabilities
- **Context Saving**: Minimal context save for performance
- **ISR Guidelines**: Short, fast handlers with deferred processing

## Memory Management

### Virtual Memory

#### Address Space Layout

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

#### Memory Protection

- **Page-Based Protection**: Hardware-enforced page-level access control
- **User/Kernel Separation**: Non-overlapping address spaces
- **NX Bit**: No-execute protection for data pages
- **ASLR**: Address Space Layout Randomization (optional)

### Paging

#### Page Table Structure

- **Multi-level Tables**: Architecture-specific depth (2-4 levels)
- **Page Sizes**: 4KB standard, optional 2MB/1GB huge pages
- **TLB Management**: Hardware TLB with software management
- **Memory Access**: Hardware page walk with software assist

#### Memory Allocation

- **Buddy Allocator**: Physical memory management
- **Slab Allocator**: Kernel object allocation
- **Memory Mapping**: VM area management
- **Memory Statistics**: Usage tracking and reporting

### TLB Management

#### TLB Operations

- **TLB Flush**: Selective and global flush operations
- **TLB Shootdown**: Multi-core TLB synchronization
- **TLB Miss Handling**: Hardware or software page walk
- **TLB Shootdown**: Efficient multi-core synchronization

### Protection Schemes

#### Memory Protection

- **Read/Write/Execute**: Individual page permissions
- **User/Kernel**: Separate privilege levels
- **Process Isolation**: Complete address space separation
- **Shared Memory**: Explicitly mapped shared regions

#### Access Control

- **Capability-Based**: Object capabilities for resource access
- **Mandatory Access Control**: System-wide security policies
- **Audit Trail**: Memory access logging (optional)

## Process/Task Model

### Task Representation

```rust
struct Task {
    id: TaskId,
    state: TaskState,
    priority: Priority,
    context: TaskContext,
    memory: MemorySpace,
    capabilities: CapabilitySpace,
    ipc_channels: Vec<ChannelId>,
}
```

### Task States

```rust
enum TaskState {
    Ready,
    Running,
    Blocked { reason: BlockReason },
    Sleeping { wake_time: u64 },
    Terminated,
}
```

### Task Scheduling

#### Scheduling Algorithm

- **Priority-Based**: Preemptive priority scheduling
- **Time Slicing**: Round-robin within same priority
- **Real-Time Support**: Earliest Deadline First (EDF) optional
- **CPU Affinity**: Optional multi-core task placement

#### Context Switching

```rust
fn switch_context(from: &mut TaskContext, to: &TaskContext) -> ! {
    // Save current task state
    save_registers(from);
    save_floating_point(from);
    
    // Load new task state
    load_registers(to);
    load_floating_point(to);
    
    // Switch page tables
    set_page_table(to.page_table);
    
    // Jump to new task
    jump_to_task(to);
}
```

### Context Switching

#### Context Save/Restore

- **Minimal Context**: Only essential registers
- **Lazy FPU**: Floating-point context save on demand
- **Cache Friendly**: Optimize for cache performance
- **Atomic Operations**: Lock-free context switching

#### Performance Optimization

- **Thread-Local Storage**: Efficient per-task data
- **Register Windows**: Architecture-specific optimizations
- **Cache Management**: Cache-aware scheduling decisions

### Synchronization Primitives

#### Mutex Implementation

```rust
struct Mutex {
    owner: Option<TaskId>,
    wait_queue: WaitQueue,
    flags: MutexFlags,
}
```

#### Semaphore and Condition Variables

- **Counting Semaphores**: Resource counting
- **Binary Semaphores**: Simple synchronization
- **Condition Variables**: Event-driven synchronization

## IPC & Synchronization

### Synchronization Primitives

#### Mutexes

- **Priority Inheritance**: Prevent priority inversion
- **Recursive Locking**: Optional reentrant mutexes
- **Timeout Support**: Deadlock prevention
- **Debug Features**: Lock ordering and deadlocks

#### Semaphores

- **Counting Semaphores**: Resource pool management
- **Binary Semaphores**: Event notification
- **Priority Queuing**: FIFO or priority-based queuing

#### Condition Variables

- **Event Synchronization**: Wait for conditions
- **Broadcast Support**: Wake all waiting tasks
- **Spurious Wakeup Handling**: Robust implementation

### Message Passing

#### Inter-Process Communication

```rust
struct Message {
    sender: TaskId,
    recipient: TaskId,
    data: Vec<u8>,
    priority: MessagePriority,
    timestamp: u64,
}
```

#### IPC Channels

- **Unidirectional Channels**: Send-only or receive-only
- **Bidirectional Channels**: Full duplex communication
- **Message Queuing**: FIFO or priority-based ordering
- **Zero-Copy**: Shared memory optimization

#### Synchronous vs Asynchronous

- **Synchronous RPC**: Blocking request-response
- **Asynchronous Messages**: Fire-and-forget
- **Notification**: Signal-based communication

## Device I/O Model

### Device Abstraction

#### Device Hierarchy

```rust
trait Device {
    fn name(&self) -> &str;
    fn device_type(&self) -> DeviceType;
    fn capabilities(&self) -> DeviceCapabilities;
    fn handle_interrupt(&self, irq: IrqNumber) -> Result<()>;
}
```

#### Device Types

- **Character Devices**: Serial, console, terminals
- **Block Devices**: Storage, memory cards
- **Network Devices**: Ethernet, wireless
- **Special Devices**: Timers, random number generators

### Interrupt Handling

#### Interrupt Controller Abstraction

```rust
trait InterruptController {
    fn enable_irq(&self, irq: IrqNumber) -> Result<()>;
    fn disable_irq(&self, irq: IrqNumber) -> Result<()>;
    fn get_pending_irqs(&self) -> Vec<IrqNumber>;
    fn end_of_interrupt(&self, irq: IrqNumber) -> Result<()>;
}
```

#### Interrupt Routing

- **Priority-Based**: Hardware interrupt priorities
- **Affinity**: Multi-core interrupt distribution
- **Load Balancing**: Dynamic load distribution
- **Affinity Change**: Runtime interrupt reassignment

### DMA

#### Direct Memory Access

- **Scatter-Gather**: Complex memory layouts
- **Circular Buffers**: Continuous data transfer
- **Cache Coherency**: Hardware cache synchronization
- **Error Handling**: Transfer failure recovery

### Device Drivers

#### Driver Architecture

```rust
trait DeviceDriver {
    fn probe(&self, device: &DeviceInfo) -> Result<bool>;
    fn attach(&self, device: &DeviceInfo) -> Result<Box<dyn Device>>;
    fn detach(&self, device: &DeviceInfo) -> Result<()>;
}
```

#### Bus Abstraction

- **PCI Express**: Modern bus standard
- **I2C**: Serial communication
- **SPI**: High-speed serial
- **Memory-Mapped I/O**: Direct device access

## Module System

### Kernel Modules

#### Module Interface

```rust
trait KernelModule {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn init(&self) -> Result<()>;
    fn fini(&self) -> Result<()>;
}
```

#### Module Management

- **Dynamic Loading**: Runtime module loading
- **Symbol Resolution**: Dynamic symbol binding
- **Dependency Management**: Module dependencies
- **Versioning**: Module version compatibility

### Symbol Resolution

#### Symbol Table Management

- **Global Symbol Table**: System-wide symbol resolution
- **Local Symbols**: Module-private symbols
- **Symbol Visibility**: Public vs private symbols
- **Weak Symbols**: Optional symbol definitions

### Module Loading/Unloading

#### Loading Process

1. **Validation**: Module integrity check
2. **Relocation**: Address space adjustment
3. **Symbol Resolution**: External symbol binding
4. **Initialization**: Module initialization routine
5. **Registration**: Service registration

#### Unloading Process

1. **Service Cleanup**: Remove registered services
2. **Resource Release**: Free allocated resources
3. **Symbol Cleanup**: Remove from symbol tables
4. **Memory Release**: Free module memory

## Security Considerations

### Memory Protection

- **Page-Level Security**: Hardware memory protection
- **Stack Protection**: Stack overflow detection
- **Heap Protection**: Heap corruption detection
- **Code Integrity**: Executable code verification

### Capability Security

- **Object Capabilities**: Secure reference passing
- **Capability Namespaces**: Hierarchical capability management
- **Revocation**: Capability revocation mechanism
- **Auditing**: Security event logging

### Attack Surface

- **Minimal Syscall Surface**: Reduced attack vectors
- **Input Validation**: Strict parameter validation
- **Buffer Overflow Protection**: Stack and heap protection
- **Return-Oriented Programming**: Control flow integrity

This specification provides the foundation for implementing Cortex-μKernel across all target architectures while maintaining the design goals of minimal size, deterministic performance, and security.