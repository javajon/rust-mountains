//! In-memory store for the mountains service.
//!
//! A Mutex around a Vec is deliberately the simplest thing that works: the point of this
//! example is gRPC and containers, not persistence. Swap this for a database and none of
//! the service code above it has to change.

use std::sync::Mutex;

use crate::mountains::Mountain;

pub struct Model {
    journal: Mutex<Vec<Mountain>>,
}

impl Default for Model {
    fn default() -> Self {
        Model { journal: Mutex::new(seed()) }
    }
}

impl Model {
    pub fn get_all(&self) -> Vec<Mountain> {
        self.journal.lock().expect("mountains lock").clone()
    }

    pub fn get(&self, id: &str) -> Option<Mountain> {
        self.journal
            .lock()
            .expect("mountains lock")
            .iter()
            .find(|m| m.id == id)
            .cloned()
    }

    /// Assigns an id, so the caller does not have to invent one.
    pub fn insert(&self, mut peak: Mountain) -> Mountain {
        peak.id = uuid::Uuid::new_v4().to_string();
        self.journal.lock().expect("mountains lock").push(peak.clone());
        peak
    }

    /// Returns None when the id is unknown, so the caller can answer NOT_FOUND rather
    /// than silently doing nothing.
    pub fn update(&self, peak: Mountain) -> Option<Mountain> {
        let mut journal = self.journal.lock().expect("mountains lock");
        let slot = journal.iter_mut().find(|m| m.id == peak.id)?;
        *slot = peak.clone();
        Some(peak)
    }

    pub fn remove(&self, id: &str) -> bool {
        let mut journal = self.journal.lock().expect("mountains lock");
        let before = journal.len();
        journal.retain(|m| m.id != id);
        journal.len() != before
    }
}

fn seed() -> Vec<Mountain> {
    vec![
        Mountain {
            id: "a48a6488-fba6-11ea-adc1-0242ac120002".to_string(),
            name: "Mt. Washington".to_string(),
            elevation: 1917,
            location: "44_16_13.8_N_71_18_11.7_W".to_string(),
        },
        Mountain {
            id: "a48a8cce-fba6-11ea-adc1-0242ac120002".to_string(),
            name: "Flatirons".to_string(),
            elevation: 2484,
            location: "39.988_N_105.293_W".to_string(),
        },
    ]
}
