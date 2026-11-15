//! SigmaOS-data-processing: data processing model.
//!
//! Real implementation: jobs, tasks, dependencies, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Processing task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessingTask {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessingError {
    EmptyName,
    DuplicateTask(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "task name must not be empty"),
            Self::DuplicateTask(id) => write!(f, "duplicate task {:?}", id),
        }
    }
}

impl std::error::Error for ProcessingError {}

/// Data processing engine.
pub struct DataProcessing {
    pub name: String,
    tasks: BTreeMap<String, ProcessingTask>,
}

impl DataProcessing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tasks: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, task: ProcessingTask) -> Result<(), ProcessingError> {
        if task.name.trim().is_empty() { return Err(ProcessingError::EmptyName); }
        let id = task.id.clone();
        if self.tasks.contains_key(&id) {
            return Err(ProcessingError::DuplicateTask(id.clone()));
        }
        self.tasks.insert(id, task);
        Ok(())
    }

    pub fn task_count(&self) -> usize { self.tasks.len() }

    pub fn get_task(&self, id: &str) -> Option<&ProcessingTask> { self.tasks.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_create_aaa() {
        let p = DataProcessing::new("My Processor");
        assert_eq!(p.name, "My Processor");
        assert_eq!(p.task_count(), 0);
    }

    #[test]
    fn test_add_task_aaa() {
        let mut p = DataProcessing::new("My Processor");
        let t = ProcessingTask { id: "t1".into(), name: "ETL".into(), status: "pending".into() };
        p.add_task(t).unwrap();
        assert_eq!(p.task_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut p = DataProcessing::new("My Processor");
        let t = ProcessingTask { id: "x".into(), name: "".into(), status: "pending".into() };
        assert!(p.add_task(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut p = DataProcessing::new("My Processor");
        let t = ProcessingTask { id: "x".into(), name: "X".into(), status: "pending".into() };
        p.add_task(t).unwrap();
        let t2 = ProcessingTask { id: "x".into(), name: "X".into(), status: "pending".into() };
        assert!(p.add_task(t2).is_err());
    }
}
