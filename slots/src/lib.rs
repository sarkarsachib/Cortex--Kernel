#![no_std]

pub const MAX_SLOTS: usize = 8;

pub struct Slot {
    id: usize,
    active: bool,
    base: usize,
    size: usize,
}

impl Slot {
    /// Creates a new slot with the given id, base, and size and initializes it inactive.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Slot::new(0, 0x1000, 0x200);
    /// assert_eq!(s.id(), 0);
    /// assert_eq!(s.base(), 0x1000);
    /// assert_eq!(s.size(), 0x200);
    /// assert!(!s.is_active());
    /// ```
    pub const fn new(id: usize, base: usize, size: usize) -> Self {
        Self {
            id,
            active: false,
            base,
            size,
        }
    }

    /// Gets the slot's identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let slot = Slot::new(3, 0x1000, 0x200);
    /// assert_eq!(slot.id(), 3);
    /// ```
    ///
    /// # Returns
    ///
    /// `usize` — the slot's identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Reports whether the slot is active.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut slot = Slot::new(0, 0x1000, 0x100);
    /// assert!(!slot.is_active());
    /// slot.activate();
    /// assert!(slot.is_active());
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if the slot is active, `false` otherwise.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// The slot's base value.
    ///
    /// # Returns
    ///
    /// The base value stored in the slot.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Slot::new(0, 0x1000, 64);
    /// assert_eq!(s.base(), 0x1000);
    /// ```
    pub fn base(&self) -> usize {
        self.base
    }

    /// Retrieve the slot's size.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Slot::new(0, 0x1000, 64);
    /// assert_eq!(s.size(), 64);
    /// ```
    ///
    /// # Returns
    ///
    /// The slot's size as a `usize`.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Marks the slot as active.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Slot::new(0, 0x1000, 0x100);
    /// s.activate();
    /// assert!(s.is_active());
    /// ```
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivates the slot, setting its active state to false.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Slot::new(0, 0, 1024);
    /// s.activate();
    /// s.deactivate();
    /// assert!(!s.is_active());
    /// ```
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

pub struct SlotManager {
    slots: [Option<Slot>; MAX_SLOTS],
    count: usize,
}

impl SlotManager {
    /// Creates an empty `SlotManager` with all slots unallocated and the slot count set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let mgr = SlotManager::new();
    /// assert_eq!(mgr.slot_count(), 0);
    /// assert!(!mgr.is_full());
    /// ```
    pub const fn new() -> Self {
        Self {
            slots: [None, None, None, None, None, None, None, None],
            count: 0,
        }
    }

    /// Allocates a new slot with the specified base and size and returns its id.
    ///
    /// # Parameters
    ///
    /// - `base`: The base address or starting value for the slot.
    /// - `size`: The size or length associated with the slot.
    ///
    /// # Returns
    ///
    /// `Some(slot_id)` containing the allocated slot's index if allocation succeeds, `None` if the manager is at capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mgr = SlotManager::new();
    /// let id = mgr.allocate(0x1000, 0x200);
    /// assert_eq!(id, Some(0));
    /// assert_eq!(mgr.slot_count(), 1);
    /// ```
    pub fn allocate(&mut self, base: usize, size: usize) -> Option<usize> {
        if self.count >= MAX_SLOTS {
            return None;
        }

        let slot_id = self.count;
        self.slots[slot_id] = Some(Slot::new(slot_id, base, size));
        self.count += 1;
        Some(slot_id)
    }

    /// Deallocates the slot at the given index, freeing its entry if present.
    ///
    /// # Parameters
    ///
    /// - `slot_id`: Index of the slot to deallocate.
    ///
    /// # Returns
    ///
    /// `true` if a slot existed at `slot_id` and was removed, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mgr = SlotManager::new();
    /// let id = mgr.allocate(0x1000, 0x200).expect("allocate");
    /// assert!(mgr.deallocate(id));
    /// assert_eq!(mgr.slot_count(), 0);
    /// ```
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

    /// Returns a reference to the slot at the given index if one is allocated there.
    ///
    /// # Parameters
    ///
    /// - `slot_id`: Index of the slot to retrieve.
    ///
    /// # Returns
    ///
    /// `Some(&Slot)` containing a reference to the slot at `slot_id` if present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mgr = SlotManager::new();
    /// let id = mgr.allocate(0x1000, 0x100).expect("allocate");
    /// let slot = mgr.get_slot(id).expect("slot present");
    /// assert_eq!(slot.id(), id);
    /// ```
    pub fn get_slot(&self, slot_id: usize) -> Option<&Slot> {
        self.slots.get(slot_id).and_then(|s| s.as_ref())
    }

    /// Gets the number of currently allocated slots.
    ///
    /// # Returns
    ///
    /// The number of allocated slots.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mgr = SlotManager::new();
    /// assert_eq!(mgr.slot_count(), 0);
    /// mgr.allocate(0, 1024);
    /// assert_eq!(mgr.slot_count(), 1);
    /// ```
    pub fn slot_count(&self) -> usize {
        self.count
    }

    /// Reports whether the manager has reached its maximum slot capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mgr = SlotManager::new();
    /// assert!(!mgr.is_full());
    ///
    /// for i in 0..MAX_SLOTS {
    ///     let _ = mgr.allocate(i, 1);
    /// }
    /// assert!(mgr.is_full());
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if the number of allocated slots is greater than or equal to `MAX_SLOTS`, `false` otherwise.
    pub fn is_full(&self) -> bool {
        self.count >= MAX_SLOTS
    }
}

impl Default for SlotManager {
    /// Creates a default SlotManager initialized with no allocated slots.
    ///
    /// # Examples
    ///
    /// ```
    /// let mgr = crate::SlotManager::default();
    /// assert_eq!(mgr.slot_count(), 0);
    /// assert!(!mgr.is_full());
    /// ```
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