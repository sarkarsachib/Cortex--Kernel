//! Timer/clock interface traits

use crate::hal::error::*;

/// Clock HAL trait - platform-independent clock interface
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

/// Default implementation for Clock HAL
impl<T: ClockHal> ClockHal for &mut T {
    type ClockId = T::ClockId;
    
    fn init(&mut self) -> HalResult<()> {
        (**self).init()
    }
    
    fn get_system_time(&self) -> u64 {
        (**self).get_system_time()
    }
    
    fn get_high_resolution_time(&self) -> u64 {
        (**self).get_high_resolution_time()
    }
    
    fn start_timer(&mut self, clock_id: Self::ClockId, 
                   frequency: u64) -> HalResult<()> {
        (**self).start_timer(clock_id, frequency)
    }
    
    fn stop_timer(&mut self, clock_id: Self::ClockId) -> HalResult<()> {
        (**self).stop_timer(clock_id)
    }
    
    fn set_timer_interrupt(&mut self, clock_id: Self::ClockId, 
                         interval_ns: u64) -> HalResult<()> {
        (**self).set_timer_interrupt(clock_id, interval_ns)
    }
    
    fn get_cpu_frequency(&self) -> u64 {
        (**self).get_cpu_frequency()
    }
    
    fn get_bus_frequency(&self) -> u64 {
        (**self).get_bus_frequency()
    }
    
    fn set_cpu_frequency(&mut self, frequency: u64) -> HalResult<()> {
        (**self).set_cpu_frequency(frequency)
    }
    
    fn enter_low_power_mode(&mut self) -> HalResult<()> {
        (**self).enter_low_power_mode()
    }
}