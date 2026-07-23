use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::domain;

#[derive(Debug)]
pub enum EngineError {
    InvalidInput,
    Cancelled,
    Internal,
}

#[derive(Default, Clone)]
pub struct CancellationRegistry {
    inner: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
}

impl CancellationRegistry {
    pub fn begin(&self, request_id: &str) -> Arc<AtomicBool> {
        let flag = Arc::new(AtomicBool::new(false));
        self.inner.lock().expect("registry poisoned")
            .insert(request_id.to_owned(), Arc::clone(&flag));
        flag
    }

    pub fn cancel(&self, request_id: &str) {
        if let Some(flag) = self.inner.lock().expect("registry poisoned").get(request_id) {
            flag.store(true, Ordering::Relaxed);
        }
    }

    pub fn finish(&self, request_id: &str) {
        self.inner.lock().expect("registry poisoned").remove(request_id);
    }
}

pub fn process(
    registry: &CancellationRegistry,
    request_id: String,
    bytes: Vec<u8>,
) -> Result<Vec<u8>, EngineError> {
    let flag = registry.begin(&request_id);
    let result = domain::process(bytes, || flag.load(Ordering::Relaxed))
        .map_err(|error| match error {
            domain::DomainError::InvalidInput => EngineError::InvalidInput,
            domain::DomainError::Cancelled => EngineError::Cancelled,
            domain::DomainError::Internal => EngineError::Internal,
        });
    registry.finish(&request_id);
    result
}
