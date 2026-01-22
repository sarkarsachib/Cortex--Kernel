//! RISC-V64 Hardware Abstraction Layer stub implementation

use crate::hal::*;

// Basic stub types for RISC-V64
pub type CpuId = u32;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;
pub type IrqNumber = u32;

// RISC-V64 CPU stub
pub struct RiscV64Cpu;

impl RiscV64Cpu {
    pub fn new() -> Self {
        Self
    }
}

impl CpuHal for RiscV64Cpu {
    type CpuId = CpuId;
    type CpuFeatures = ();
    
    fn cpu_id(&self) -> Self::CpuId {
        0
    }
    
    fn cpu_count(&self) -> usize {
        1
    }
    
    fn features(&self) -> Self::CpuFeatures {
        ()
    }
    
    fn has_feature(&self, _feature: CpuFeature) -> bool {
        true // Stub - assume all features available
    }
    
    fn enable_interrupts(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn disable_interrupts(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn is_interrupts_enabled(&self) -> bool {
        true // Stub
    }
    
    fn idle(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn halt(&self) -> HalResult<()> {
        loop {}
    }
    
    fn icache_invalidate(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn dcache_invalidate(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn dcache_flush(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn memory_barrier(&self) -> HalResult<()> {
        Ok(())
    }
    
    fn instruction_barrier(&self) -> HalResult<()> {
        Ok(())
    }
}

// RISC-V64 HAL stub
pub struct RiscV64Hal {
    cpu: RiscV64Cpu,
}

impl RiscV64Hal {
    pub fn new() -> Self {
        Self {
            cpu: RiscV64Cpu::new(),
        }
    }
}

impl Hal for RiscV64Hal {
    type Cpu = RiscV64Cpu;
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