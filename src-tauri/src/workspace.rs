use crate::models::{brief_case::BriefCase, task::Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub task_list: Vec<Task>,
    pub current_task: usize,
}

impl WorkspaceState {
    pub fn new(task_list: Vec<Task>) -> Self {
        Self { task_list, current_task: 0 }
    }
}
