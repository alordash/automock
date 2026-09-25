use std::sync::atomic::{AtomicUsize, Ordering};

// Helper trait for callbacks verification.
// Leaking for ability to `Copy` so no need to create clones for moving them in closures.
#[derive(Copy, Clone)]
pub struct Counter(&'static AtomicUsize);
#[allow(unused)]
impl Counter {
    pub fn new() -> Self {
        Self(Box::leak(Box::new(AtomicUsize::new(0))))
    }
    pub fn inc(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    #[allow(unused)] // idk why cargo check it's not used anywhere despite it being used a lot
    pub fn double_inc(&self) {
        self.0.fetch_add(2, Ordering::Relaxed);
    }
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}
