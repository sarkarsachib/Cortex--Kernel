#![no_std]

pub const MAX_SLOTS: usize = 8;

pub struct Slot {
    id: usize,
    active: bool,
    base: usize,
    size: usize,
}

impl Slot {
    pub const fn new(id: usize, base: usize, size: usize) -> Self {
        Self {
            id,
            active: false,
            base,
            size,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn base(&self) -> usize {
        self.base
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

pub struct SlotManager {
    slots: [Option<Slot>; MAX_SLOTS],
    count: usize,
}

impl SlotManager {
    pub const fn new() -> Self {
        Self {
            slots: [None, None, None, None, None, None, None, None],
            count: 0,
        }
    }

    pub fn allocate(&mut self, base: usize, size: usize) -> Option<usize> {
        if self.count >= MAX_SLOTS {
            return None;
        }

        let slot_id = self.count;
        self.slots[slot_id] = Some(Slot::new(slot_id, base, size));
        self.count += 1;
        Some(slot_id)
    }

    pub fn deallocate(&mut self, slot_id: usize) -> bool {
        if slot_id >= MAX_SLOTS {
            return false;
        }

        if self.slots[slot_id].is_some() {
            self.slots[slot_id] = None;
            self.count -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_slot(&self, slot_id: usize) -> Option<&Slot> {
        self.slots.get(slot_id).and_then(|s| s.as_ref())
    }

    pub fn slot_count(&self) -> usize {
        self.count
    }

    pub fn is_full(&self) -> bool {
        self.count >= MAX_SLOTS
    }
}

impl Default for SlotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_creation() {
        let slot = Slot::new(0, 0x1000, 0x100);
        assert_eq!(slot.id(), 0);
        assert_eq!(slot.base(), 0x1000);
        assert_eq!(slot.size(), 0x100);
        assert!(!slot.is_active());
    }

    #[test]
    fn test_slot_activation() {
        let mut slot = Slot::new(0, 0x1000, 0x100);
        slot.activate();
        assert!(slot.is_active());
        slot.deactivate();
        assert!(!slot.is_active());
    }

    #[test]
    fn test_slot_manager_creation() {
        let manager = SlotManager::new();
        assert_eq!(manager.slot_count(), 0);
        assert!(!manager.is_full());
    }

    #[test]
    fn test_slot_allocate() {
        let mut manager = SlotManager::new();
        let id = manager.allocate(0x1000, 0x100);
        assert!(id.is_some());
        assert_eq!(id.unwrap(), 0);
        assert_eq!(manager.slot_count(), 1);
    }

    #[test]
    fn test_slot_deallocate() {
        let mut manager = SlotManager::new();
        let id = manager.allocate(0x1000, 0x100).unwrap();
        assert!(manager.deallocate(id));
        assert_eq!(manager.slot_count(), 0);
    }

    #[test]
    fn test_max_slots() {
        let mut manager = SlotManager::new();
        for i in 0..MAX_SLOTS {
            assert!(manager.allocate(i * 0x1000, 0x100).is_some());
        }
        assert!(manager.is_full());
        assert!(manager.allocate(0x8000, 0x100).is_none());
    }
}
