use crate::models::{brief_case::BriefCase, task::Task};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub task_list: Vec<Task>,
    pub current_task: usize,
    pub is_running: bool,
    pub visited_briefcase_ids: HashSet<Uuid>,
    pub current_profile_window_label: Option<String>,
    pub current_profile_id: Option<Uuid>,
}

impl WorkspaceState {
    pub fn new(task_list: Vec<Task>) -> Self {
        Self {
            task_list,
            current_task: 0,
            is_running: false,
            visited_briefcase_ids: HashSet::new(),
            current_profile_window_label: None,
            current_profile_id: None,
        }
    }

    pub fn mark_briefcase_visited(&mut self, briefcase_id: Uuid) {
        self.visited_briefcase_ids.insert(briefcase_id);
    }

    pub fn is_briefcase_visited(&self, briefcase_id: Uuid) -> bool {
        self.visited_briefcase_ids.contains(&briefcase_id)
    }

    pub fn get_current_task(&self) -> Option<&Task> {
        self.task_list.get(self.current_task)
    }

    pub fn get_current_briefcase(&self) -> Option<&BriefCase> {
        self.get_current_task()
            .and_then(|t| t.related_brief_cases.as_ref())
            .and_then(|bcs| bcs.first())
    }

    pub fn get_visited_count(&self) -> usize {
        self.visited_briefcase_ids.len()
    }

    pub fn get_total_briefcase_count(&self) -> usize {
        self.task_list
            .iter()
            .filter_map(|t| t.related_brief_cases.as_ref())
            .map(|bcs| bcs.len())
            .sum()
    }

    pub fn set_current_profile_label(&mut self, label: String) {
        self.current_profile_window_label = Some(label);
    }

    pub fn get_current_profile_label(&self) -> Option<&String> {
        self.current_profile_window_label.as_ref()
    }

    pub fn set_current_profile_id(&mut self, profile_id: Uuid) {
        self.current_profile_id = Some(profile_id);
    }

    pub fn get_current_profile_id(&self) -> Option<Uuid> {
        self.current_profile_id
    }
}
