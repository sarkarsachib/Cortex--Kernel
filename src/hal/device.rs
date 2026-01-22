//! Device bus and driver traits

use crate::hal::error::*;

/// Device types
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

/// Resource types
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    MemoryMappedIo,
    PortIo,
    Interrupt,
    DmaChannel,
}

/// Device resource structure
#[derive(Debug, Clone)]
pub struct DeviceResource {
    pub resource_type: ResourceType,
    pub address: usize,
    pub size: usize,
}

/// Device capabilities
#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub can_read: bool,
    pub can_write: bool,
    pub can_interrupt: bool,
    pub can_dma: bool,
}

/// Device information structure
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub capabilities: DeviceCapabilities,
    pub resources: Vec<DeviceResource>,
}

/// Device HAL trait - platform-independent device interface
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

/// Default implementation for Device HAL
impl<T: DeviceHal> DeviceHal for &T {
    type DeviceId = T::DeviceId;
    type BusType = T::BusType;
    
    fn enumerate_devices(&self) -> HalResult<Vec<DeviceInfo>> {
        (**self).enumerate_devices()
    }
    
    fn find_device(&self, name: &str) -> HalResult<Self::DeviceId> {
        (**self).find_device(name)
    }
    
    fn device_info(&self, id: Self::DeviceId) -> HalResult<DeviceInfo> {
        (**self).device_info(id)
    }
    
    fn enable_device(&self, id: Self::DeviceId) -> HalResult<()> {
        (**self).enable_device(id)
    }
    
    fn disable_device(&self, id: Self::DeviceId) -> HalResult<()> {
        (**self).disable_device(id)
    }
    
    fn scan_bus(&self, bus: Self::BusType) -> HalResult<()> {
        (**self).scan_bus(bus)
    }
    
    fn read_config_space(&self, device: Self::DeviceId, 
                        offset: u16) -> HalResult<u32> {
        (**self).read_config_space(device, offset)
    }
    
    fn write_config_space(&self, device: Self::DeviceId, 
                         offset: u16, value: u32) -> HalResult<()> {
        (**self).write_config_space(device, offset, value)
    }
}