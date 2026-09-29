use crate::infrastructure::*;
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct IdMockData {
    id: usize,
    inner: RwLock<MockData>,
}

impl Default for IdMockData {
    fn default() -> Self {
        Self {
            id: MOCK_ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            inner: Default::default(),
        }
    }
}

impl IdMockData {
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline(always)]
    pub(crate) fn inner(&self) -> &RwLock<MockData> {
        &self.inner
    }
}
