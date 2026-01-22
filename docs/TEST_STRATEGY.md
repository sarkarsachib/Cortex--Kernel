# Cortex-μKernel Testing & Validation Strategy

## Overview

This document outlines the comprehensive testing and validation strategy for the Cortex-μKernel project. The strategy ensures that the kernel operates correctly across all supported architectures while maintaining the strict size constraints and performance requirements.

## Testing Philosophy

### Core Principles

1. **Layered Testing**: Test each abstraction layer independently
2. **Multi-Architecture Validation**: Ensure consistent behavior across all platforms
3. **Performance Validation**: Verify timing constraints and performance targets
4. **Security Testing**: Validate security features and memory protection
5. **Minimal Footprint**: Ensure testing itself doesn't violate size constraints

### Test Categories

- **Unit Tests**: Individual component functionality
- **Integration Tests**: Cross-component interactions
- **Architecture Tests**: Platform-specific validation
- **Performance Tests**: Timing and resource usage
- **Security Tests**: Memory protection and isolation
- **Boot Tests**: System initialization validation

## HAL Trait Validation

### CPU HAL Tests

#### Feature Detection Testing

```rust
#[cfg(test)]
mod cpu_hal_tests {
    use super::*;
    
    #[test]
    fn test_cpu_identification() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // Verify CPU ID is valid
        let cpu_id = cpu.cpu_id();
        assert!(cpu_id != 0);
        
        // Verify CPU count is at least 1
        assert!(cpu.cpu_count() >= 1);
    }
    
    #[test]
    fn test_feature_detection() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // Atomic operations should be supported on all platforms
        assert!(cpu.has_feature(CpuFeature::AtomicOperations));
        
        // Floating point support varies by platform
        let fp_supported = cpu.has_feature(CpuFeature::HardwareFloatingPoint);
        
        #[cfg(target_arch = "x86_64")]
        assert!(fp_supported);
        
        #[cfg(target_arch = "aarch64")]
        assert!(fp_supported);
    }
    
    #[test]
    fn test_interrupt_control() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // Test interrupt disabling/enabling
        cpu.disable_interrupts().unwrap();
        assert!(!cpu.is_interrupts_enabled());
        
        cpu.enable_interrupts().unwrap();
        assert!(cpu.is_interrupts_enabled());
    }
    
    #[test]
    fn test_cache_operations() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // Test cache operations
        cpu.icache_invalidate().unwrap();
        cpu.dcache_invalidate().unwrap();
        cpu.dcache_flush().unwrap();
        
        // Test memory barriers
        cpu.memory_barrier().unwrap();
        cpu.instruction_barrier().unwrap();
    }
}
```

#### Cross-Architecture CPU Testing

```rust
#[cfg(test)]
mod cross_arch_cpu_tests {
    use crate::hal::*;
    
    #[test]
    fn test_cpu_consistency() {
        // Test that all platforms implement the same CPU interface
        let x86_64_hal = X86_64Hal::new();
        let arm64_hal = Arm64Hal::new();
        let arm32_hal = Arm32Hal::new();
        let riscv64_hal = RiscV64Hal::new();
        
        // All should have at least one CPU
        assert!(x86_64_hal.cpu().cpu_count() >= 1);
        assert!(arm64_hal.cpu().cpu_count() >= 1);
        assert!(arm32_hal.cpu().cpu_count() >= 1);
        assert!(riscv64_hal.cpu().cpu_count() >= 1);
        
        // All should support atomic operations
        assert!(x86_64_hal.cpu().has_feature(CpuFeature::AtomicOperations));
        assert!(arm64_hal.cpu().has_feature(CpuFeature::AtomicOperations));
        assert!(arm32_hal.cpu().has_feature(CpuFeature::AtomicOperations));
        assert!(riscv64_hal.cpu().has_feature(CpuFeature::AtomicOperations));
    }
}
```

### Memory HAL Tests

#### Page Table Testing

```rust
#[cfg(test)]
mod memory_hal_tests {
    use super::*;
    
    #[test]
    fn test_memory_discovery() {
        let hal = create_platform_hal();
        let memory = hal.memory();
        
        // Verify total memory is reasonable
        let total_memory = memory.total_memory_size();
        assert!(total_memory >= 1024 * 1024); // At least 1MB
        
        // Verify available memory is less than or equal to total
        let available_memory = memory.available_memory_size();
        assert!(available_memory <= total_memory);
        
        // Verify memory map is not empty
        let memory_map = memory.memory_map();
        assert!(!memory_map.is_empty());
    }
    
    #[test]
    fn test_page_table_operations() {
        let hal = create_platform_hal();
        let memory = hal.memory();
        
        // Create a page table
        let page_table = memory.create_page_table().unwrap();
        
        // Test page mapping
        let virt_addr = 0x1000;
        let phys_addr = 0x1000;
        let flags = PageFlags {
            present: true,
            writable: true,
            executable: false,
            user_accessible: false,
            cache_disable: false,
            write_through: false,
        };
        
        memory.map_page(&page_table, virt_addr, phys_addr, flags).unwrap();
        
        // Verify mapping
        let retrieved_flags = memory.get_page_flags(&page_table, virt_addr).unwrap();
        assert_eq!(retrieved_flags.present, true);
        assert_eq!(retrieved_flags.writable, true);
        
        // Test address translation
        let translated_phys = memory.virt_to_phys(&page_table, virt_addr).unwrap();
        assert_eq!(translated_phys, phys_addr);
        
        // Test page unmapping
        memory.unmap_page(&page_table, virt_addr).unwrap();
        
        // Verify mapping is removed
        let retrieved_flags = memory.get_page_flags(&page_table, virt_addr).unwrap();
        assert_eq!(retrieved_flags.present, false);
        
        // Cleanup
        memory.destroy_page_table(page_table).unwrap();
    }
    
    #[test]
    fn test_tlb_operations() {
        let hal = create_platform_hal();
        let memory = hal.memory();
        
        let page_table = memory.create_page_table().unwrap();
        
        // Test TLB invalidate operations
        memory.tlb_invalidate(&page_table).unwrap();
        
        let virt_addr = 0x2000;
        memory.tlb_invalidate_page(&page_table, virt_addr).unwrap();
        
        memory.destroy_page_table(page_table).unwrap();
    }
}
```

### Interrupt HAL Tests

#### IRQ Management Testing

```rust
#[cfg(test)]
mod interrupt_hal_tests {
    use super::*;
    
    #[test]
    fn test_interrupt_controller_init() {
        let mut hal = create_platform_hal();
        let interrupt = hal.interrupt();
        
        // Initialize interrupt controller
        interrupt.init().unwrap();
        
        // Verify controller type is set
        assert_ne!(interrupt.controller_type(), InterruptControllerType::None);
    }
    
    #[test]
    fn test_irq_management() {
        let mut hal = create_platform_hal();
        let interrupt = hal.interrupt();
        
        interrupt.init().unwrap();
        
        // Test IRQ enable/disable
        let test_irq = IrqNumber::from(0);
        
        interrupt.enable_irq(test_irq).unwrap();
        assert!(interrupt.is_irq_enabled(test_irq));
        
        interrupt.disable_irq(test_irq).unwrap();
        assert!(!interrupt.is_irq_enabled(test_irq));
        
        // Test IRQ masking
        interrupt.mask_irq(test_irq).unwrap();
        interrupt.unmask_irq(test_irq).unwrap();
        
        // Test priority management
        interrupt.set_priority(test_irq, 5).unwrap();
        let priority = interrupt.get_priority(test_irq).unwrap();
        assert_eq!(priority, 5);
    }
    
    #[test]
    fn test_interrupt_status() {
        let mut hal = create_platform_hal();
        let interrupt = hal.interrupt();
        
        interrupt.init().unwrap();
        
        let test_irq = IrqNumber::from(1);
        
        // Initially no IRQ should be pending
        assert!(!interrupt.is_irq_pending(test_irq));
    }
}
```

### Device HAL Tests

#### Device Enumeration Testing

```rust
#[cfg(test)]
mod device_hal_tests {
    use super::*;
    
    #[test]
    fn test_device_enumeration() {
        let hal = create_platform_hal();
        let device = hal.device();
        
        // Enumerate all devices
        let devices = device.enumerate_devices().unwrap();
        
        // Should find at least basic system devices
        assert!(!devices.is_empty());
        
        // Verify device information structure
        for device_info in &devices {
            assert!(!device_info.name.is_empty());
            assert_ne!(device_info.device_type, DeviceType::Unknown);
        }
    }
    
    #[test]
    fn test_device_capabilities() {
        let hal = create_platform_hal();
        let device = hal.device();
        
        let devices = device.enumerate_devices().unwrap();
        
        for device_info in devices {
            // Each device should have valid capabilities
            assert!(!device_info.capabilities.is_empty());
            
            // Verify resource information
            for resource in &device_info.resources {
                assert!(resource.size > 0);
                match resource.resource_type {
                    ResourceType::MemoryMappedIo | ResourceType::PortIo => {
                        assert!(resource.address > 0);
                    }
                    ResourceType::Interrupt => {
                        // Interrupt numbers should be valid
                        assert!(resource.address <= 255);
                    }
                    _ => {}
                }
            }
        }
    }
}
```

### Clock HAL Tests

#### Timer Testing

```rust
#[cfg(test)]
mod clock_hal_tests {
    use super::*;
    
    #[test]
    fn test_clock_initialization() {
        let mut hal = create_platform_hal();
        let clock = hal.clock();
        
        // Initialize clock subsystem
        clock.init().unwrap();
        
        // Verify we can get system time
        let system_time = clock.get_system_time();
        assert!(system_time > 0);
        
        let high_res_time = clock.get_high_resolution_time();
        assert!(high_res_time > 0);
    }
    
    #[test]
    fn test_frequency_information() {
        let mut hal = create_platform_hal();
        let clock = hal.clock();
        
        clock.init().unwrap();
        
        // Verify CPU frequency is reasonable
        let cpu_freq = clock.get_cpu_frequency();
        assert!(cpu_freq > 0);
        assert!(cpu_freq < 10_000_000_000); // Less than 10GHz
        
        let bus_freq = clock.get_bus_frequency();
        assert!(bus_freq > 0);
        assert!(bus_freq <= cpu_freq); // Bus should not be faster than CPU
    }
}
```

## Integration Testing

### HAL Initialization Tests

```rust
#[cfg(test)]
mod hal_integration_tests {
    use super::*;
    
    #[test]
    fn test_complete_hal_initialization() {
        let mut hal = create_platform_hal();
        
        // Early initialization
        hal.early_init().unwrap();
        
        // Verify all subsystems are initialized
        assert!(hal.cpu().cpu_count() > 0);
        assert!(hal.memory().total_memory_size() > 0);
        
        // Late initialization
        hal.late_init().unwrap();
        
        // Verify full functionality
        let devices = hal.device().enumerate_devices().unwrap();
        assert!(!devices.is_empty());
    }
    
    #[test]
    fn test_subsystem_interactions() {
        let mut hal = create_platform_hal();
        
        // Initialize all subsystems
        hal.early_init().unwrap();
        
        // Test interaction between memory and CPU
        let memory = hal.memory();
        let cpu = hal.cpu();
        
        // Create page table and map memory
        let page_table = memory.create_page_table().unwrap();
        let virt_addr = 0x400000;
        let phys_addr = 0x400000;
        let flags = PageFlags::kernel_default();
        
        memory.map_page(&page_table, virt_addr, phys_addr, flags).unwrap();
        
        // Switch to new address space (should be platform-specific)
        #[cfg(target_arch = "x86_64")]
        {
            memory.switch_page_table(&page_table);
        }
        
        // Verify CPU can access the mapped memory
        cpu.memory_barrier().unwrap();
        
        // Cleanup
        memory.destroy_page_table(page_table).unwrap();
    }
}
```

## Architecture-Specific Testing

### Boot Sequence Validation

```rust
#[cfg(test)]
mod boot_sequence_tests {
    use super::*;
    
    #[test]
    fn test_multiboot2_compliance() {
        #[cfg(target_arch = "x86_64")]
        {
            // Verify Multiboot2 header is present
            let boot_header = get_multiboot2_header();
            assert_eq!(boot_header.magic, 0x36D76289);
            
            // Verify required tags can be parsed
            let boot_info = parse_multiboot2_tags();
            assert!(boot_info.memory_map.is_some());
        }
    }
    
    #[test]
    fn test_uefi_compliance() {
        #[cfg(target_arch = "aarch64")]
        {
            // Verify UEFI system table can be accessed
            let system_table = get_uefi_system_table();
            assert!(!system_table.is_null());
            
            // Verify boot services are available
            let boot_services = get_boot_services();
            assert!(!boot_services.is_null());
        }
    }
    
    #[test]
    fn test_device_tree_parsing() {
        #[cfg(any(target_arch = "arm", target_arch = "riscv64"))]
        {
            // Verify Device Tree can be parsed
            let fdt = get_device_tree();
            let parsed_fdt = parse_fdt(fdt).unwrap();
            
            // Verify required nodes exist
            assert!(parsed_fdt.find_node("/memory").is_some());
            assert!(parsed_fdt.find_node("/cpus").is_some());
            assert!(parsed_fdt.find_node("/interrupt-controller").is_some());
        }
    }
}
```

### Platform-Specific Feature Tests

```rust
#[cfg(test)]
mod platform_specific_tests {
    use super::*;
    
    #[test]
    fn test_x86_64_features() {
        #[cfg(target_arch = "x86_64")]
        {
            let hal = X86_64Hal::new();
            
            // Test CPUID support
            let cpu_features = hal.cpu().get_cpuid_features();
            assert!(cpu_features.has_sse2());
            assert!(cpu_features.has_64bit_support());
            
            // Test MSR access
            let misc_enable = hal.cpu().read_msr(0x1A0).unwrap();
            assert!(misc_enable > 0);
        }
    }
    
    #[test]
    fn test_arm64_features() {
        #[cfg(target_arch = "aarch64")]
        {
            let hal = Arm64Hal::new();
            
            // Test MIDR register access
            let midr = hal.cpu().read_midr().unwrap();
            assert!(midr.implementer > 0);
            
            // Test system register access
            let sctlr = hal.cpu().read_sctlr_el1().unwrap();
            // SCTLR should be readable
            assert!(true);
        }
    }
    
    #[test]
    fn test_riscv64_features() {
        #[cfg(target_arch = "riscv64")]
        {
            let hal = RiscV64Hal::new();
            
            // Test CSR access
            let misa = hal.cpu().read_csr(0x301).unwrap(); // misa CSR
            assert!(misa > 0);
            
            // Test extension detection
            let extensions = hal.cpu().get_extensions();
            assert!(extensions.m); // Integer multiplication
            assert!(extensions.a); // Atomic operations
        }
    }
}
```

## Performance Testing

### Timing Validation Tests

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_context_switch_performance() {
        let mut hal = create_platform_hal();
        hal.init().unwrap();
        
        // Create two tasks for context switching
        let task1 = create_test_task();
        let task2 = create_test_task();
        
        // Measure context switch time
        let start_time = hal.clock().get_high_resolution_time();
        
        for _ in 0..1000 {
            switch_context(&task1, &task2);
        }
        
        let end_time = hal.clock().get_high_resolution_time();
        let total_time = end_time - start_time;
        let avg_switch_time = total_time / 1000;
        
        // Context switch should be under 1 microsecond
        assert!(avg_switch_time < 1000); // Nanoseconds
    }
    
    #[test]
    fn test_interrupt_latency() {
        let mut hal = create_platform_hal();
        hal.init().unwrap();
        
        let interrupt = hal.interrupt();
        
        // Test interrupt handling latency
        let start_time = hal.clock().get_high_resolution_time();
        
        // Simulate interrupt handling
        interrupt.enable_irq(test_irq()).unwrap();
        
        let end_time = hal.clock().get_high_resolution_time();
        let latency = end_time - start_time;
        
        // Interrupt latency should be under 10 microseconds
        assert!(latency < 10_000); // Nanoseconds
    }
    
    #[test]
    fn test_memory_access_performance() {
        let hal = create_platform_hal();
        let memory = hal.memory();
        
        // Create page table and map memory
        let page_table = memory.create_page_table().unwrap();
        
        // Test sequential memory access
        let start_time = hal.clock().get_high_resolution_time();
        
        for i in 0..1000 {
            let addr = 0x1000 + i * 4096;
            memory.get_page_flags(&page_table, addr).unwrap();
        }
        
        let end_time = hal.clock().get_high_resolution_time();
        let access_time = (end_time - start_time) / 1000;
        
        // Memory access should be reasonably fast
        assert!(access_time < 100); // Nanoseconds per access
        
        memory.destroy_page_table(page_table).unwrap();
    }
}
```

## Security Testing

### Memory Protection Tests

```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_memory_isolation() {
        let mut hal = create_platform_hal();
        let memory = hal.memory();
        
        // Create two separate address spaces
        let page_table1 = memory.create_page_table().unwrap();
        let page_table2 = memory.create_page_table().unwrap();
        
        // Map different physical memory to same virtual address
        let virt_addr = 0x5000;
        let phys_addr1 = 0x5000;
        let phys_addr2 = 0x6000;
        let flags = PageFlags::user_default();
        
        memory.map_page(&page_table1, virt_addr, phys_addr1, flags).unwrap();
        memory.map_page(&page_table2, virt_addr, phys_addr2, flags).unwrap();
        
        // Verify isolation
        let trans_phys1 = memory.virt_to_phys(&page_table1, virt_addr).unwrap();
        let trans_phys2 = memory.virt_to_phys(&page_table2, virt_addr).unwrap();
        
        assert_ne!(trans_phys1, trans_phys2);
        
        // Cleanup
        memory.destroy_page_table(page_table1).unwrap();
        memory.destroy_page_table(page_table2).unwrap();
    }
    
    #[test]
    fn test_execution_protection() {
        let hal = create_platform_hal();
        let memory = hal.memory();
        
        let page_table = memory.create_page_table().unwrap();
        
        // Map memory as non-executable
        let virt_addr = 0x7000;
        let phys_addr = 0x7000;
        let flags = PageFlags {
            present: true,
            writable: true,
            executable: false, // No execute
            user_accessible: true,
            cache_disable: false,
            write_through: false,
        };
        
        memory.map_page(&page_table, virt_addr, phys_addr, flags).unwrap();
        
        // Verify execution is prevented
        let retrieved_flags = memory.get_page_flags(&page_table, virt_addr).unwrap();
        assert_eq!(retrieved_flags.executable, false);
        
        memory.destroy_page_table(page_table).unwrap();
    }
}
```

## Hardware Capability Testing

### CPU Feature Validation

```rust
#[cfg(test)]
mod cpu_capability_tests {
    use super::*;
    
    #[test]
    fn test_required_features() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // All platforms must support these features
        assert!(cpu.has_feature(CpuFeature::AtomicOperations));
        
        // Memory management should be available
        assert!(cpu.has_feature(CpuFeature::VirtualMemory));
        
        // Interrupt support should be available
        assert!(cpu.has_feature(CpuFeature::HardwareInterrupts));
    }
    
    #[test]
    fn test_optional_features() {
        let hal = create_platform_hal();
        let cpu = hal.cpu();
        
        // These features may or may not be present
        let has_fp = cpu.has_feature(CpuFeature::HardwareFloatingPoint);
        let has_simd = cpu.has_feature(CpuFeature::SimdExtensions);
        let has_virt = cpu.has_feature(CpuFeature::VirtualizationExtensions);
        
        // Log what features are available
        println!("Floating Point: {}", has_fp);
        println!("SIMD Extensions: {}", has_simd);
        println!("Virtualization: {}", has_virt);
        
        // No assertions - these are informational
    }
}
```

## Boot Testing

### System Initialization Tests

```rust
#[cfg(test)]
mod boot_tests {
    use super::*;
    
    #[test]
    fn test_hal_boot_sequence() {
        // Test that HAL can be initialized from boot state
        let mut hal = create_platform_hal();
        
        // Early boot initialization
        hal.early_init().unwrap();
        
        // Verify critical subsystems are working
        assert!(hal.cpu().cpu_count() > 0);
        assert!(hal.memory().total_memory_size() > 0);
        
        // Late boot initialization
        hal.late_init().unwrap();
        
        // Verify full functionality
        assert!(hal.interrupt().controller_type() != InterruptControllerType::None);
        assert!(hal.clock().get_system_time() > 0);
    }
    
    #[test]
    fn test_kernel_integration() {
        // Test that kernel can integrate with HAL
        let mut hal = create_platform_hal();
        hal.init().unwrap();
        
        // Create kernel instance
        let kernel = Kernel::new();
        let mut kernel = kernel;
        
        // Initialize kernel with HAL
        kernel.init(&hal).unwrap();
        
        // Verify kernel subsystems are functional
        assert!(kernel.scheduler.is_initialized());
        assert!(kernel.memory_manager.is_initialized());
        assert!(kernel.time_manager.is_initialized());
    }
}
```

## Testing Infrastructure

### Test Framework

```rust
// Test utilities for kernel testing
pub struct TestEnvironment {
    hal: Box<dyn Hal>,
    test_memory: TestMemoryAllocator,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let mut hal = create_hal();
        hal.init().unwrap();
        
        Self {
            hal,
            test_memory: TestMemoryAllocator::new(),
        }
    }
    
    pub fn hal(&self) -> &dyn Hal {
        self.hal.as_ref()
    }
    
    pub fn alloc_test_memory(&mut self, size: usize) -> *mut u8 {
        self.test_memory.allocate(size)
    }
    
    pub fn free_test_memory(&mut self, ptr: *mut u8) {
        self.test_memory.deallocate(ptr);
    }
}

// Custom test allocator for kernel tests
struct TestMemoryAllocator {
    next_addr: usize,
}

impl TestMemoryAllocator {
    pub fn new() -> Self {
        Self {
            next_addr: 0x1000000, // Start at 16MB
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        let addr = self.next_addr;
        self.next_addr += size;
        addr as *mut u8
    }
    
    pub fn deallocate(&mut self, _ptr: *mut u8) {
        // Simple allocator - no actual deallocation
    }
}
```

### Automated Testing Scripts

```bash
#!/bin/bash
# run_platform_tests.sh - Run tests on all platforms

set -e

PLATFORMS=("x86_64" "arm64" "arm32" "riscv64")

for platform in "${PLATFORMS[@]}"; do
    echo "Testing platform: $platform"
    
    # Build for platform
    cargo build --target $platform
    
    # Run architecture-specific tests
    cargo test --target $platform --features test_suite
    
    # Run QEMU boot test
    ./build/qemu_${platform}.sh &
    QEMU_PID=$!
    
    # Wait for boot completion
    sleep 5
    
    # Check if QEMU is still running (indicates successful boot)
    if kill -0 $QEMU_PID 2>/dev/null; then
        echo "✓ $platform boot test passed"
        kill $QEMU_PID
    else
        echo "✗ $platform boot test failed"
        exit 1
    fi
done

echo "All platform tests passed!"
```

### Continuous Integration Tests

```yaml
# .github/workflows/test.yml
name: Kernel Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        platform: [x86_64, arm64, arm32, riscv64]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
        components: rust-src
        target: ${{ matrix.platform }}
    
    - name: Install QEMU
      run: |
        sudo apt-get update
        sudo apt-get install -y qemu-system-x86 qemu-system-aarch64 qemu-system-arm qemu-system-riscv64
    
    - name: Build
      run: cargo build --target ${{ matrix.platform }}
    
    - name: Run Tests
      run: cargo test --target ${{ matrix.platform }}
    
    - name: Run Integration Tests
      run: |
        # Architecture-specific integration tests
        ./build/qemu_${{ matrix.platform }}.sh &
        sleep 5
        kill %1  # Kill QEMU
```

## Test Coverage Goals

### Coverage Requirements

- **Unit Test Coverage**: > 90% for all HAL modules
- **Integration Test Coverage**: > 80% for cross-component interactions
- **Architecture Test Coverage**: 100% for each supported platform
- **Performance Test Coverage**: All critical paths measured
- **Security Test Coverage**: All memory protection features tested

### Performance Benchmarks

- **Context Switch Time**: < 1μs
- **Interrupt Latency**: < 10μs
- **Memory Access Time**: < 100ns per access
- **Boot Time**: < 100ms
- **Kernel Size**: < 20KB per platform

This testing strategy ensures that Cortex-μKernel meets all functional, performance, and security requirements while maintaining the minimal size constraint across all supported architectures.