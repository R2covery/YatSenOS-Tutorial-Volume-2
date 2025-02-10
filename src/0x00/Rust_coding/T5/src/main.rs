use std::sync::atomic::{AtomicU16, Ordering};

struct UniqueId(u16);

impl UniqueId {
    static mut NEXT_ID: u16 = 0;

    fn new() -> Self {
        unsafe {
            let id = UniqueId::NEXT_ID;
            UniqueId::NEXT_ID = UniqueId::NEXT_ID.wrapping_add(1);
            UniqueId(id)
        }
    }
}

impl PartialEq for UniqueId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UniqueId {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_id() {
        let id1 = UniqueId::new();
        let id2 = UniqueId::new();
        assert_ne!(id1, id2);
    }
}