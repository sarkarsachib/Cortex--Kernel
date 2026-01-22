//! Interrupt controller traits

use crate::hal::error::*;

/// Interrupt controller types
#[derive(Debug, Clone, Copy)]
pub enum InterruptControllerType {
    Apic,           // x86_64
    Gic,            // ARM64/ARM32
    Nvic,           // ARM32 Cortex-M
    Clic,           // RISC-V
    Plic,           // RISC-V
}

/// Interrupt HAL trait - platform-independent interrupt interface
pub trait InterruptHal {
    type IrqNumber: Copy;
    
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
    fn set_priority(&mut self, irq: Self::IrqNumber, priority: u8) -> HalResult<()>;
    fn get_priority(&self, irq: Self::IrqNumber) -> HalResult<u8>;
}

/// Default implementation for Interrupt HAL
impl<T: InterruptHal> InterruptHal for &mut T {
    type IrqNumber = T::IrqNumber;
    
    fn init(&mut self) -> HalResult<()> {
        (**self).init()
    }
    
    fn controller_type(&self) -> InterruptControllerType {
        (**self).controller_type()
    }
    
    fn enable_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()> {
        (**self).enable_irq(irq)
    }
    
    fn disable_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()> {
        (**self).disable_irq(irq)
    }
    
    fn mask_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()> {
        (**self).mask_irq(irq)
    }
    
    fn unmask_irq(&mut self, irq: Self::IrqNumber) -> HalResult<()> {
        (**self).unmask_irq(irq)
    }
    
    fn is_irq_enabled(&self, irq: Self::IrqNumber) -> bool {
        (**self).is_irq_enabled(irq)
    }
    
    fn is_irq_pending(&self, irq: Self::IrqNumber) -> bool {
        (**self).is_irq_pending(irq)
    }
    
    fn end_of_interrupt(&mut self, irq: Self::IrqNumber) -> HalResult<()> {
        (**self).end_of_interrupt(irq)
    }
    
    fn set_priority(&mut self, irq: Self::IrqNumber, priority: u8) -> HalResult<()> {
        (**self).set_priority(irq, priority)
    }
    
    fn get_priority(&self, irq: Self::IrqNumber) -> HalResult<u8> {
        (**self).get_priority(irq)
    }
}