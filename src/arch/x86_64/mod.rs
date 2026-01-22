//! x86_64 Hardware Abstraction Layer implementation

use crate::hal::*;

// x86_64 specific types
pub type CpuId = u32;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;
pub type IrqNumber = u8;

// x86_64 CPU features structure
#[derive(Clone, Copy, Debug)]
pub struct X86_64CpuFeatures {
    pub has_sse2: bool,
    pub has_64bit: bool,
}

// x86_64 page table structure (simplified)
pub struct X86_64PageTable {
    _private: (),
}

// x86_64 CPU implementation
pub struct X86_64Cpu {
    cpu_id: CpuId,
    cpu_count: usize,
    features: X86_64CpuFeatures,
}

impl X86_64Cpu {
    pub fn new() -> Self {
        Self {
            cpu_id: 0,
            cpu_count: 1,
            features: X86_64CpuFeatures {
                has_sse2: true,
                has_64bit: true,
            },
        }
    }
}

impl CpuHal for X86_64Cpu {
    type CpuId = CpuId;
    type CpuFeatures = X86_64CpuFeatures;

    fn cpu_id(&self) -> Self::CpuId {
        self.cpu_id
    }

    fn cpu_count(&self) -> usize {
        self.cpu_count
    }

    fn features(&self) -> Self::CpuFeatures {
        self.features
    }

    fn has_feature(&self, feature: CpuFeature) -> bool {
        match feature {
            CpuFeature::AtomicOperations => true,
            CpuFeature::HardwareFloatingPoint => self.features.has_sse2,
            CpuFeature::VirtualMemory => true,
            CpuFeature::HardwareInterrupts => true,
            _ => false,
        }
    }

    fn enable_interrupts(&self) -> HalResult<()> {
        unsafe { core::arch::asm!("sti"); }
        Ok(())
    }

    fn disable_interrupts(&self) -> HalResult<()> {
        unsafe { core::arch::asm!("cli"); }
        Ok(())
    }

    fn is_interrupts_enabled(&self) -> bool {
        let flags: u64;
        unsafe { core::arch::asm!("pushfq; popq {}", out(reg) flags); }
        (flags & 0x200) != 0
    }

    fn idle(&self) -> HalResult<()> {
        unsafe { core::arch::asm!("hlt"); }
        Ok(())
    }

    fn halt(&self) -> HalResult<()> {
        loop {
            unsafe { core::arch::asm!("hlt"); }
        }
    }

    fn icache_invalidate(&self) -> HalResult<()> {
        // x86_64 instruction cache is self-invalidating
        Ok(())
    }

    fn dcache_invalidate(&self) -> HalResult<()> {
        // x86_64 data cache coherency is handled by hardware
        Ok(())
    }

    fn dcache_flush(&self) -> HalResult<()> {
        Ok(())
    }

    fn memory_barrier(&self) -> HalResult<()> {
        unsafe { core::arch::asm!("mfence"); }
        Ok(())
    }

    fn instruction_barrier(&self) -> HalResult<()> {
        unsafe { core::arch::asm!("lfence"); }
        Ok(())
    }
}

// x86_64 HAL implementation
pub struct X86_64Hal {
    cpu: X86_64Cpu,
}

impl X86_64Hal {
    pub fn new() -> Self {
        Self {
            cpu: X86_64Cpu::new(),
        }
    }
}

impl Hal for X86_64Hal {
    type Cpu = X86_64Cpu;
    type Memory = ();
    type Interrupt = ();
    type Device = ();
    type Clock = ();
    
    fn cpu(&self) -> &Self::Cpu {
        &self.cpu
    }
    
    fn memory(&self) -> &Self::Memory {
        &()
    }
    
    fn interrupt(&self) -> &Self::Interrupt {
        &()
    }
    
    fn device(&self) -> &Self::Device {
        &()
    }
    
    fn clock(&self) -> &Self::Clock {
        &()
    }
    
    fn early_init(&mut self) -> HalResult<()> {
        Ok(())
    }
    
    fn late_init(&mut self) -> HalResult<()> {
        Ok(())
    }
}