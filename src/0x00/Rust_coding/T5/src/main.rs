use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Debug, PartialEq, Eq)]
struct UniqueId(u16);

impl UniqueId {
    fn new() -> Self {
        static COUNTER: AtomicU16 = AtomicU16::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        UniqueId(id)
    }
}

#[test]
fn test_unique_id() {
    let id1 = UniqueId::new();
    let id2 = UniqueId::new();
    assert_ne!(id1, id2);
}

fn main() {
    let id1 = UniqueId::new();
    let id2 = UniqueId::new();
    println!("{:?}, {:?}", id1, id2);
}