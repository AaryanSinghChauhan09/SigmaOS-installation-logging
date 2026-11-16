// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-installation-logging: installation logging component model.
//!
//! Real implementation: loggings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logging {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallationLoggingError {
    EmptyLoggingName,
    DuplicateLogging(String),
    EmptyEventSubject,
}

impl fmt::Display for InstallationLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoggingName => write!(f, "logging name must not be empty"),
            Self::DuplicateLogging(id) => write!(f, "duplicate logging {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for InstallationLoggingError {}

pub struct InstallationLogging {
    pub name: String,
    loggings: BTreeMap<String, Logging>,
    events: BTreeMap<String, Event>,
}

impl InstallationLogging {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            loggings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_logging(&mut self, item: Logging) -> Result<(), InstallationLoggingError> {
        if item.name.trim().is_empty() {
            return Err(InstallationLoggingError::EmptyLoggingName);
        }
        let id = item.id.clone();
        if self.loggings.contains_key(&id) {
            return Err(InstallationLoggingError::DuplicateLogging(id.clone()));
        }
        self.loggings.insert(id, item);
        Ok(())
    }

    pub fn logging_count(&self) -> usize {
        self.loggings.len()
    }

    pub fn get_logging(&self, id: &str) -> Option<&Logging> {
        self.loggings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), InstallationLoggingError> {
        if event.subject.trim().is_empty() {
            return Err(InstallationLoggingError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_logging(id: &str, name: &str) -> Logging {
        Logging {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = InstallationLogging::new("InstallationLogging");
        assert_eq!(m.name, "InstallationLogging");
        assert_eq!(m.logging_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_logging() {
        let mut m = InstallationLogging::new("InstallationLogging");
        m.add_logging(sample_logging("a1", "Alpha")).unwrap();
        assert_eq!(m.logging_count(), 1);
        assert!(m.get_logging("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = InstallationLogging::new("InstallationLogging");
        assert!(m.add_logging(sample_logging("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = InstallationLogging::new("InstallationLogging");
        m.add_logging(sample_logging("x", "One")).unwrap();
        assert!(m.add_logging(sample_logging("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = InstallationLogging::new("InstallationLogging");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
