//! Memory management traits

use crate::hal::error::*;

/// Memory types
#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    Available,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
}

/// Memory region structure
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub addr: usize,
    pub size: usize,
    pub region_type: MemoryType,
}

/// Page flags for memory protection
#[derive(Debug, Clone, Copy)]
pub struct PageFlags {
    pub present: bool,
    pub writable: bool,
    pub executable: bool,
    pub user_accessible: bool,
    pub cache_disable: bool,
    pub write_through: bool,
}

impl PageFlags {
    /// Default flags for kernel memory
    pub fn kernel_default() -> Self {
        Self {
            present: true,
            writable: true,
            executable: true,
            user_accessible: false,
            cache_disable: false,
            write_through: false,
        }
    }
    
    /// Default flags for user memory
    pub fn user_default() -> Self {
        Self {
            present: true,
            writable: true,
            executable: false,
            user_accessible: true,
            cache_disable: false,
            write_through: false,
        }
    }
    
    /// No access flags
    pub fn none() -> Self {
        Self {
            present: false,
            writable: false,
            executable: false,
            user_accessible: false,
            cache_disable: false,
            write_through: false,
        }
    }
}

/// Memory HAL trait - platform-independent memory interface
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
    
    // Platform-specific operations
    fn enable_paging(&self) -> HalResult<()>;
    fn disable_paging(&self) -> HalResult<()>;
}

/// Default implementation for Memory HAL
impl<T: MemoryHal> MemoryHal for &T {
    type PageTable = T::PageTable;
    type PhysicalAddress = T::PhysicalAddress;
    type VirtualAddress = T::VirtualAddress;
    
    fn total_memory_size(&self) -> usize {
        (**self).total_memory_size()
    }
    
    fn available_memory_size(&self) -> usize {
        (**self).available_memory_size()
    }
    
    fn memory_map(&self) -> &[MemoryRegion] {
        (**self).memory_map()
    }
    
    fn create_page_table(&self) -> HalResult<Self::PageTable> {
        (**self).create_page_table()
    }
    
    fn destroy_page_table(&self, pt: Self::PageTable) -> HalResult<()> {
        (**self).destroy_page_table(pt)
    }
    
    fn map_page(&self, pt: &Self::PageTable, 
                virt: Self::VirtualAddress, 
                phys: Self::PhysicalAddress,
                flags: PageFlags) -> HalResult<()> {
        (**self).map_page(pt, virt, phys, flags)
    }
    
    fn unmap_page(&self, pt: &Self::PageTable, 
                  virt: Self::VirtualAddress) -> HalResult<()> {
        (**self).unmap_page(pt, virt)
    }
    
    fn get_page_flags(&self, pt: &Self::PageTable, 
                     virt: Self::VirtualAddress) -> HalResult<PageFlags> {
        (**self).get_page_flags(pt, virt)
    }
    
    fn virt_to_phys(&self, pt: &Self::PageTable, 
                   virt: Self::VirtualAddress) -> HalResult<Self::PhysicalAddress> {
        (**self).virt_to_phys(pt, virt)
    }
    
    fn phys_to_virt(&self, pt: &Self::PageTable, 
                   phys: Self::PhysicalAddress) -> HalResult<Self::VirtualAddress> {
        (**self).phys_to_virt(pt, phys)
    }
    
    fn tlb_invalidate(&self, pt: &Self::PageTable) -> HalResult<()> {
        (**self).tlb_invalidate(pt)
    }
    
    fn tlb_invalidate_page(&self, pt: &Self::PageTable, 
                          virt: Self::VirtualAddress) -> HalResult<()> {
        (**self).tlb_invalidate_page(pt, virt)
    }
    
    fn enable_paging(&self) -> HalResult<()> {
        (**self).enable_paging()
    }
    
    fn disable_paging(&self) -> HalResult<()> {
        (**self).disable_paging()
    }
}