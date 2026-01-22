# Cortex-μKernel Hardware Abstraction Layer (HAL) Architecture

## Overview

The Hardware Abstraction Layer (HAL) provides a unified interface for hardware-specific operations across all supported architectures (x86_64, ARM64, ARM32, RISC-V64). The HAL enables the kernel to remain architecture-independent while accessing platform-specific hardware features.

## HAL Layer Responsibilities

### Primary Responsibilities

1. **CPU Detection & Initialization**
   - CPU feature detection and capability reporting
   - CPU-specific initialization sequences
   - Performance monitoring and configuration

2. **Memory Management**
   - Early memory discovery and initialization
   - Page table setup and management
   - Memory protection and access control

3. **Interrupt Management**
   - Interrupt controller initialization
   - Interrupt routing and priority management
   - Exception handling setup

4. **Device Enumeration**
   - Bus scanning and device discovery
   - Resource allocation and management
   - Device capability reporting

5. **Clock & Timer Management**
   - System timer initialization
   - High-resolution timer support
   - CPU frequency and power management

## Architecture-Specific Modules

### Directory Structure

```
src/
├── hal/
│   ├── mod.rs              # Core HAL traits and types
│   ├── cpu.rs              # CPU interface traits
│   ├── interrupt.rs        # Interrupt controller traits
│   ├── memory.rs           # Memory management traits
│   ├── device.rs           # Device bus traits
│   ├── clock.rs            # Timer/clock traits
│   └── error.rs            # HAL error types
├── arch/
│   ├── x86_64/
│   │   ├── mod.rs          # x86_64 HAL implementation
│   │   ├── cpu.rs          # CPU detection (CPUID)
│   │   ├── interrupt.rs    # APIC/IDT management
│   │   ├── memory.rs       # Page tables, E820
│   │   ├── io.rs           # Port I/O, MSR access
│   │   └── boot.rs         # Multiboot2 interface
│   ├── arm64/
│   │   ├── mod.rs          # ARM64 HAL implementation
│   │   ├── cpu.rs          # MIDR, system registers
│   │   ├── interrupt.rs    # GICv2/v3 management
│   │   ├── memory.rs       # Page tables, MMU
│   │   ├── io.rs           # MMIO access
│   │   └── boot.rs         # UEFI interface
│   ├── arm32/
│   │   ├── mod.rs          # ARM32 HAL implementation
│   │   ├── cpu.rs          # CP15 registers
│   │   ├── interrupt.rs    # NVIC management
│   │   ├── memory.rs       # Page tables, MPU
│   │   ├── io.rs           # MMIO access
│   │   └── boot.rs         # Device tree interface
│   └── riscv64/
│       ├── mod.rs          # RISC-V64 HAL implementation
│       ├── cpu.rs          # CSR access, extensions
│       ├── interrupt.rs    # CLIC/PLIC management
│       ├── memory.rs       # Page tables, PMP
│       ├── io.rs           # MMIO access
│       └── boot.rs         # Device tree interface
```

## Architecture Traits

### Core HAL Trait Hierarchy

```rust
// Core error type for all HAL operations
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

// Result type alias
pub type HalResult<T> = Result<T, HalError>;

// Main HAL trait - platform entry point
pub trait Hal {
    type Cpu: CpuHal;
    type Memory: MemoryHal;
    type Interrupt: InterruptHal;
    type Device: DeviceHal;
    type Clock: ClockHal;
    
    fn cpu(&self) -> &Self::Cpu;
    fn memory(&self) -> &Self::Memory;
    fn interrupt(&self) -> &Self::Interrupt;
    fn device(&self) -> &Self::Device;
    fn clock(&self) -> &Self::Clock;
    
    fn early_init(&mut self) -> HalResult<()>;
    fn late_init(&mut self) -> HalResult<()>;
}
```

### CPU HAL Trait

```rust
pub trait CpuHal {
    type CpuId: Copy;
    type CpuFeatures: Copy;
    
    // CPU identification
    fn cpu_id(&self) -> Self::CpuId;
    fn cpu_count(&self) -> usize;
    
    // CPU features and capabilities
    fn features(&self) -> Self::CpuFeatures;
    fn has_feature(&self, feature: CpuFeature) -> bool;
    
    // CPU control
    fn enable_interrupts(&self) -> HalResult<()>;
    fn disable_interrupts(&self) -> HalResult<()>;
    fn is_interrupts_enabled(&self) -> bool;
    
    // Power management
    fn idle(&self) -> HalResult<()>;
    fn halt(&self) -> HalResult<()>;
    
    // Cache management
    fn icache_invalidate(&self) -> HalResult<()>;
    fn dcache_invalidate(&self) -> HalResult<()>;
    fn dcache_flush(&self) -> HalResult<()>;
    
    // Memory barrier operations
    fn memory_barrier(&self) -> HalResult<()>;
    fn instruction_barrier(&self) -> HalResult<()>;
}

// CPU feature flags (architecture-agnostic)
#[derive(Debug, Clone, Copy)]
pub enum CpuFeature {
    AtomicOperations,
    HardwareFloatingPoint,
    SimdExtensions,
    VirtualizationExtensions,
    CryptoExtensions,
}
```

### Memory HAL Trait

```rust
pub trait MemoryHal {
    type PageTable;
    type PhysicalAddress: Copy;
    type VirtualAddress: Copy;
    
    // Memory discovery
    fn total_memory_size(&self) -> usize;
    fn available_memory_size(&self) -> usize;
    fn memory_map(&self) -> &[MemoryRegion];
    
    // Page table management
    fn create_page_table(&self) -> HalResult<Self::PageTable>;
    fn destroy_page_table(&self, pt: Self::PageTable) -> HalResult<()>;
    
    // Virtual memory operations
    fn map_page(&self, pt: &Self::PageTable, 
                virt: Self::VirtualAddress, 
                phys: Self::PhysicalAddress,
                flags: PageFlags) -> HalResult<()>;
    fn unmap_page(&self, pt: &Self::PageTable, 
                  virt: Self::VirtualAddress) -> HalResult<()>;
    fn get_page_flags(&self, pt: &Self::PageTable, 
                     virt: Self::VirtualAddress) -> HalResult<PageFlags>;
    
    // Address translation
    fn virt_to_phys(&self, pt: &Self::PageTable, 
                   virt: Self::VirtualAddress) -> HalResult<Self::PhysicalAddress>;
    fn phys_to_virt(&self, pt: &Self::PageTable, 
                   phys: Self::PhysicalAddress) -> HalResult<Self::VirtualAddress>;
    
    // TLB operations
    fn tlb_invalidate(&self, pt: &Self::PageTable) -> HalResult<()>;
    fn tlb_invalidate_page(&self, pt: &Self::PageTable, 
                          virt: Self::VirtualAddress) -> HalResult<()>;
}

// Memory types and flags
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub addr: usize,
    pub size: usize,
    pub region_type: MemoryType,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    Available,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
}

#[derive(Debug, Clone, Copy)]
pub struct PageFlags {
    pub present: bool,
    pub writable: bool,
    pub executable: bool,
    pub user_accessible: bool,
    pub cache_disable: bool,
    pub write_through: bool,
}
```

### Interrupt HAL Trait

```rust
pub trait InterruptHal {
    type IrqNumber: Copy;
    type InterruptController;
    
    // Interrupt controller management
    fn init(&mut self) -> HalResult<()>;
    fn controller_type(&self) -> InterruptControllerType;
    
    // IRQ management
    fn enable_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()>;
    fn disable_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()>;
    fn mask_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()>;
    fn unmask_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()>;
    
    // Interrupt status
    fn is_irq_enabled(&self, irq: Self::IrqNumber) -> bool;
    fn is_irq_pending(&self, irq: Self::IrqNumber) -> bool;
    
    // End of interrupt
    fn end_of_interrupt(&mut self, irq: Self::IrqNumber) -> HalResult<()>;
    
    // Priority management
    fn set_priority(&mut self, irq: Self::IrqNumber, 
                   priority: u8) -> HalResult<()>;
    fn get_priority(&self, irq: Self::IrqNumber) -> HalResult<u8>;
}

#[derive(Debug, Clone, Copy)]
pub enum InterruptControllerType {
    Apic,           // x86_64
    Gic,            // ARM64/ARM32
    Nvic,           // ARM32 Cortex-M
    Clic,           // RISC-V
    Plic,           // RISC-V
}
```

### Device HAL Trait

```rust
pub trait DeviceHal {
    type DeviceId: Copy;
    type BusType;
    
    // Device enumeration
    fn enumerate_devices(&self) -> HalResult<Vec<DeviceInfo>>;
    fn find_device(&self, name: &str) -> HalResult<Self::DeviceId>;
    
    // Device management
    fn device_info(&self, id: Self::DeviceId) -> HalResult<DeviceInfo>;
    fn enable_device(&self, id: Self::DeviceId) -> HalResult<()>;
    fn disable_device(&self, id: Self::DeviceId) -> HalResult<()>;
    
    // Bus operations
    fn scan_bus(&self, bus: Self::BusType) -> HalResult<()>;
    fn read_config_space(&self, device: Self::DeviceId, 
                        offset: u16) -> HalResult<u32>;
    fn write_config_space(&self, device: Self::DeviceId, 
                         offset: u16, value: u32) -> HalResult<()>;
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub capabilities: DeviceCapabilities,
    pub resources: Vec<DeviceResource>,
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Timer,
    Serial,
    Network,
    Storage,
    Gpio,
    I2c,
    Spi,
    Usb,
    Display,
    Audio,
}

#[derive(Debug, Clone)]
pub struct DeviceResource {
    pub resource_type: ResourceType,
    pub address: usize,
    pub size: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    MemoryMappedIo,
    PortIo,
    Interrupt,
    DmaChannel,
}
```

### Clock HAL Trait

```rust
pub trait ClockHal {
    type ClockId: Copy;
    
    // System clock management
    fn init(&mut self) -> HalResult<()>;
    fn get_system_time(&self) -> u64;
    fn get_high_resolution_time(&self) -> u64;
    
    // Timer operations
    fn start_timer(&mut self, clock_id: Self::ClockId, 
                   frequency: u64) -> HalResult<()>;
    fn stop_timer(&mut self, clock_id: Self::ClockId) -> HalResult<()>;
    fn set_timer_interrupt(&mut self, clock_id: Self::ClockId, 
                         interval_ns: u64) -> HalResult<()>;
    
    // Clock information
    fn get_cpu_frequency(&self) -> u64;
    fn get_bus_frequency(&self) -> u64;
    
    // Power management
    fn set_cpu_frequency(&mut self, frequency: u64) -> HalResult<()>;
    fn enter_low_power_mode(&mut self) -> HalResult<()>;
}
```

## Boot Sequence

### Architecture-Specific Boot Protocols

#### x86_64 Boot Sequence

```rust
pub fn boot() -> HalResult<()> {
    // 1. Multiboot2 compliance
    let boot_info = multiboot2::BootInfo::parse(boot_info_addr)?;
    
    // 2. Early HAL initialization
    let mut hal = X86_64Hal::new();
    hal.early_init()?;
    
    // 3. Memory initialization
    hal.memory().init_from_multiboot(&boot_info)?;
    
    // 4. CPU initialization
    hal.cpu().init()?;
    
    // 5. Interrupt initialization
    hal.interrupt().init()?;
    
    // 6. Device enumeration
    hal.device().enumerate_devices()?;
    
    // 7. Clock initialization
    hal.clock().init()?;
    
    // 8. Late initialization
    hal.late_init()?;
    
    Ok(())
}
```

#### ARM64 Boot Sequence

```rust
pub fn boot() -> HalResult<()> {
    // 1. UEFI interface
    let system_table = get_uefi_system_table();
    
    // 2. Early HAL initialization
    let mut hal = Arm64Hal::new();
    hal.early_init()?;
    
    // 3. Memory initialization
    hal.memory().init_from_uefi(&system_table)?;
    
    // 4. CPU initialization
    hal.cpu().init()?;
    
    // 5. Interrupt initialization (GIC)
    hal.interrupt().init()?;
    
    // 6. Device enumeration
    hal.device().enumerate_devices()?;
    
    // 7. Clock initialization
    hal.clock().init()?;
    
    // 8. Late initialization
    hal.late_init()?;
    
    Ok(())
}
```

#### Generic Boot Flow

```
Bootloader Entry
    ↓
Early HAL Init
    ↓
Memory Discovery & Init
    ↓
CPU Detection & Init
    ↓
Interrupt Controller Init
    ↓
Device Enumeration
    ↓
Clock/Timer Init
    ↓
Late HAL Init
    ↓
Kernel Main
```

## CPU Capabilities

### Feature Detection

#### x86_64 CPUID

```rust
pub struct X86_64CpuInfo {
    pub vendor: String,
    pub model: u32,
    pub stepping: u32,
    pub features: X86_64Features,
}

#[derive(Clone, Copy)]
pub struct X86_64Features {
    pub sse2: bool,
    pub sse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub avx512: bool,
    pub aes: bool,
    pub rdrand: bool,
    pub fsgsbase: bool,
}
```

#### ARM64 MIDR

```rust
pub struct Arm64CpuInfo {
    pub implementer: u8,
    pub variant: u8,
    pub part: u16,
    pub revision: u8,
    pub features: Arm64Features,
}

#[derive(Clone, Copy)]
pub struct Arm64Features {
    pub fp: bool,           // Floating point
    pub asimd: bool,         // Advanced SIMD
    pub evtstrm: bool,       // Event stream
    pub aes: bool,          // AES instructions
    pub pmull: bool,        // Polynomial multiplication
    pub sha1: bool,         // SHA1 instructions
    pub sha256: bool,       // SHA256 instructions
    pub sha512: bool,       // SHA512 instructions
    pub crc32: bool,        // CRC32 instructions
    pub lse: bool,          // Atomic operations
}
```

#### RISC-V Extensions

```rust
#[derive(Clone, Copy)]
pub struct RiscVFeatures {
    pub m: bool,            // Integer multiplication/division
    pub a: bool,            // Atomic operations
    pub f: bool,            // Single-precision floating point
    pub d: bool,            // Double-precision floating point
    pub c: bool,            // Compressed instructions
    pub h: bool,            // Hypervisor support
    pub s: bool,            // Supervisor mode
    pub u: bool,            // User mode
    pub v: bool,            // Vector extensions
}
```

## Interrupt Handling

### Controller Initialization

#### APIC (x86_64)

```rust
impl InterruptHal for X86_64Hal {
    fn init(&mut self) -> HalResult<()> {
        // 1. Local APIC initialization
        self.local_apic.init()?;
        
        // 2. I/O APIC initialization
        self.io_apic.init()?;
        
        // 3. Interrupt routing setup
        self.setup_irq_routing()?;
        
        // 4. Enable interrupts
        self.cpu().enable_interrupts()?;
        
        Ok(())
    }
}
```

#### GIC (ARM64)

```rust
impl InterruptHal for Arm64Hal {
    fn init(&mut self) -> HalResult<()> {
        // 1. GIC distributor initialization
        self.gic_distributor.init()?;
        
        // 2. GIC CPU interface initialization
        self.gic_cpu_interface.init()?;
        
        // 3. Interrupt routing
        self.setup_spi_routing()?;
        
        // 4. Enable interrupts
        self.cpu().enable_interrupts()?;
        
        Ok(())
    }
}
```

## Memory Initialization

### Early Page Table Setup

#### Multi-Level Page Tables

```rust
pub struct PageTable {
    entries: Vec<PageTableEntry>,
    level: usize,
    virtual_address_bits: usize,
}

impl PageTable {
    pub fn new(level: usize, virt_bits: usize) -> Self {
        let entries = vec![PageTableEntry::empty(); 1 << (virt_bits / (4 * level))];
        Self { entries, level, virtual_address_bits: virt_bits }
    }
}
```

### Memory Discovery

#### x86_64 E820

```rust
pub struct E820MemoryMap {
    entries: Vec<E820Entry>,
}

#[repr(C)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub entry_type: u32,
}
```

#### ARM64/ARM32 Device Tree

```rust
pub struct DeviceTree {
    pub root: DeviceTreeNode,
}

pub struct DeviceTreeNode {
    pub name: String,
    pub properties: HashMap<String, Vec<u8>>,
    pub children: Vec<DeviceTreeNode>,
}
```

## Device Detection

### Bus Scanning

#### PCI Express Enumeration

```rust
pub fn scan_pci_bus() -> HalResult<Vec<PciDevice>> {
    let mut devices = Vec::new();
    
    // Scan PCI configuration space
    for bus in 0..256 {
        for device in 0..32 {
            for function in 0..8 {
                let addr = PciAddress::new(bus, device, function);
                
                if let Ok(device_info) = read_pci_device_info(addr) {
                    if device_info.vendor_id != 0xFFFF {
                        devices.push(device_info);
                    }
                }
            }
        }
    }
    
    Ok(devices)
}
```

#### Device Tree Parsing

```rust
pub fn parse_device_tree(dt: &DeviceTree) -> HalResult<Vec<DeviceInfo>> {
    let mut devices = Vec::new();
    
    fn traverse_node(node: &DeviceTreeNode, devices: &mut Vec<DeviceInfo>) -> HalResult<()> {
        // Extract device information from node properties
        if let Some(device_type) = node.properties.get("device_type") {
            let device_info = DeviceInfo::from_device_tree_node(node)?;
            devices.push(device_info);
        }
        
        // Recursively traverse children
        for child in &node.children {
            traverse_node(child, devices)?;
        }
        
        Ok(())
    }
    
    traverse_node(&dt.root, &mut devices)?;
    Ok(devices)
}
```

## Platform Initialization Priority

### Initialization Order

1. **Critical Hardware** (Immediate)
   - CPU detection and initialization
   - Early memory setup
   - Basic console output

2. **Memory Subsystem** (Early)
   - Physical memory discovery
   - Page table initialization
   - Memory allocation setup

3. **Interrupt System** (Early)
   - Interrupt controller initialization
   - Exception vector setup
   - Basic interrupt routing

4. **Timer System** (Early)
   - System timer initialization
   - Scheduling timer setup
   - High-resolution timer support

5. **Device System** (Late)
   - Bus scanning and enumeration
   - Device driver initialization
   - Resource allocation

6. **Power Management** (Late)
   - CPU frequency scaling
   - Power state management
   - Thermal management

### Dependency Graph

```
CPU Init → Memory Init → Interrupt Init → Timer Init → Device Init
     ↓         ↓            ↓             ↓           ↓
   Early    Early        Early         Early       Late
```

## Error Handling

### HAL Error Types

```rust
#[derive(Debug)]
pub enum HalError {
    UnsupportedFeature,
    InitializationFailed(String),
    ResourceUnavailable,
    InvalidParameter,
    Timeout(Duration),
    PermissionDenied,
    DeviceNotFound,
    MemoryAllocationFailed,
    InvalidAddress,
    AlignmentError,
}

impl HalError {
    pub fn is_critical(&self) -> bool {
        matches!(self, 
            HalError::InitializationFailed(_) | 
            HalError::MemoryAllocationFailed |
            HalError::InvalidAddress)
    }
}
```

### Error Recovery

```rust
pub fn init_with_recovery<H: Hal>(&mut self, hal: &mut H) -> HalResult<()> {
    // Try initialization
    if let Err(error) = hal.early_init() {
        // Log error
        log::error!("HAL early init failed: {:?}", error);
        
        // Attempt recovery for non-critical errors
        if !error.is_critical() {
            self.attempt_error_recovery(&error)?;
            hal.early_init()
        } else {
            Err(error)
        }
    } else {
        Ok(())
    }
}
```

## Testing Strategy

### HAL Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_feature_detection() {
        let hal = X86_64Hal::new();
        let cpu = hal.cpu();
        
        assert!(cpu.cpu_count() > 0);
        assert!(cpu.has_feature(CpuFeature::AtomicOperations));
    }
    
    #[test]
    fn test_memory_mapping() {
        let hal = X86_64Hal::new();
        let memory = hal.memory();
        
        let page_table = memory.create_page_table().unwrap();
        assert!(page_table.is_valid());
        
        memory.destroy_page_table(page_table).unwrap();
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use crate::hal::*;
    
    #[test]
    fn test_hal_initialization() {
        let mut hal = create_platform_hal();
        
        // Test early initialization
        assert!(hal.early_init().is_ok());
        
        // Test late initialization
        assert!(hal.late_init().is_ok());
        
        // Verify all subsystems are functional
        assert!(hal.cpu().cpu_count() > 0);
        assert!(hal.memory().total_memory_size() > 0);
        assert!(hal.interrupt().controller_type() != InterruptControllerType::None);
    }
}
```

This HAL architecture provides the foundation for implementing platform-specific hardware support while maintaining a clean, architecture-independent kernel interface.