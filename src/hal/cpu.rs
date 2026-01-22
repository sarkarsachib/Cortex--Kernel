//! CPU interface traits and types

use crate::hal::error::*;

/// CPU feature flags (architecture-agnostic)
#[derive(Debug, Clone, Copy)]
pub enum CpuFeature {
    AtomicOperations,
    HardwareFloatingPoint,
    SimdExtensions,
    VirtualizationExtensions,
    CryptoExtensions,
    VirtualMemory,
    HardwareInterrupts,
}

/// CPU HAL trait - platform-independent CPU interface
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

/// Default implementation for CPU HAL
impl<T: CpuHal> CpuHal for &T {
    type CpuId = T::CpuId;
    type CpuFeatures = T::CpuFeatures;
    
    fn cpu_id(&self) -> Self::CpuId {
        (**self).cpu_id()
    }
    
    fn cpu_count(&self) -> usize {
        (**self).cpu_count()
    }
    
    fn features(&self) -> Self::CpuFeatures {
        (**self).features()
    }
    
    fn has_feature(&self, feature: CpuFeature) -> bool {
        (**self).has_feature(feature)
    }
    
    fn enable_interrupts(&self) -> HalResult<()> {
        (**self).enable_interrupts()
    }
    
    fn disable_interrupts(&self) -> HalResult<()> {
        (**self).disable_interrupts()
    }
    
    fn is_interrupts_enabled(&self) -> bool {
        (**self).is_interrupts_enabled()
    }
    
    fn idle(&self) -> HalResult<()> {
        (**self).idle()
    }
    
    fn halt(&self) -> HalResult<()> {
        (**self).halt()
    }
    
    fn icache_invalidate(&self) -> HalResult<()> {
        (**self).icache_invalidate()
    }
    
    fn dcache_invalidate(&self) -> HalResult<()> {
        (**self).dcache_invalidate()
    }
    
    fn dcache_flush(&self) -> HalResult<()> {
        (**self).dcache_flush()
    }
    
    fn memory_barrier(&self) -> HalResult<()> {
        (**self).memory_barrier()
    }
    
    fn instruction_barrier(&self) -> HalResult<()> {
        (**self).instruction_barrier()
    }
}