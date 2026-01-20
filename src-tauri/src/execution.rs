use crate::models::Task;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Represents a single execution step: posting a comment from a specific BriefCase on a specific Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Index of the task in the task list
    pub task_index: usize,
    /// Index of the BriefCase in the task's related_brief_cases vector
    pub briefcase_index: usize,
    /// The profile ID this BriefCase belongs to (for loading the correct browser profile)
    pub profile_id: Uuid,
    /// The URL of the social media post
    pub link: String,
}

/// Represents execution steps grouped by profile for optimized profile switching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileExecution {
    /// The profile ID for this group of executions
    pub profile_id: Uuid,
    /// All execution steps that can be done on this profile
    pub steps: Vec<ExecutionStep>,
}

/// Optimized execution plan that groups BriefCases by profile to minimize profile switches
///
/// The execution plan creates an optimized sequence where:
/// 1. All executions on a single profile are grouped together
/// 2. When processing a profile, we find all tasks that have BriefCases on that profile
/// 3. We only switch profiles when all tasks on the current profile are complete
///
/// This is much more efficient than switching profiles for each individual comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Execution groups organized by profile ID (BTreeMap for consistent ordering)
    pub profile_executions: BTreeMap<Uuid, ProfileExecution>,
    /// Flattened list of all steps in execution order (for easy iteration)
    execution_order: Vec<ExecutionStep>,
    /// Current position in the execution order
    current_step_index: usize,
    /// Set of visited BriefCase IDs to track progress
    visited_briefcase_ids: Vec<Uuid>,
}

impl ExecutionPlan {
    /// Creates a new optimized execution plan from a list of tasks
    ///
    /// # Algorithm:
    /// 1. For each task, iterate through all its BriefCases
    /// 2. Group executions by profile_id to minimize profile switches
    /// 3. Create a flattened execution order that respects profile grouping
    ///
    /// # Arguments
    /// * `tasks` - The list of tasks to create a plan for
    ///
    /// # Returns
    /// A new ExecutionPlan optimized for minimal profile switching
    pub fn new(tasks: &[Task]) -> Self {
        let mut profile_executions: BTreeMap<Uuid, ProfileExecution> = BTreeMap::new();

        // Group execution steps by profile ID
        for (task_index, task) in tasks.iter().enumerate() {
            for (briefcase_index, briefcase) in task.related_brief_cases.iter().enumerate() {
                let step = ExecutionStep {
                    task_index,
                    briefcase_index,
                    profile_id: briefcase.profile_id,
                    link: task.link.clone(),
                };

                profile_executions
                    .entry(briefcase.profile_id)
                    .or_insert_with(|| ProfileExecution {
                        profile_id: briefcase.profile_id,
                        steps: Vec::new(),
                    })
                    .steps
                    .push(step);
            }
        }

        // Create flattened execution order by iterating through profiles
        let execution_order: Vec<ExecutionStep> = profile_executions
            .values()
            .flat_map(|pe| pe.steps.clone())
            .collect();

        Self {
            profile_executions,
            execution_order,
            current_step_index: 0,
            visited_briefcase_ids: Vec::new(),
        }
    }

    /// Gets the next execution step in the sequence
    ///
    /// # Returns
    /// Some(ExecutionStep) if there are more steps, None if complete
    pub fn next(&mut self) -> Option<ExecutionStep> {
        if self.current_step_index >= self.execution_order.len() {
            return None;
        }

        let step = self.execution_order[self.current_step_index].clone();
        self.current_step_index += 1;
        Some(step)
    }

    /// Checks if the next step requires a profile change
    ///
    /// # Returns
    /// true if profile should change, false if same profile or no more steps
    pub fn should_change_profile(&self) -> bool {
        if self.current_step_index >= self.execution_order.len() {
            return false;
        }

        if self.current_step_index == 0 {
            return true;
        }

        // Compare current profile with next profile
        let current_profile_id = self.execution_order[self.current_step_index - 1].profile_id;
        let next_profile_id = self.execution_order[self.current_step_index].profile_id;

        current_profile_id != next_profile_id
    }

    /// Marks a BriefCase as visited for progress tracking
    ///
    /// # Arguments
    /// * `briefcase_id` - The ID of the BriefCase to mark as visited
    pub fn mark_visited(&mut self, briefcase_id: Uuid) {
        if !self.visited_briefcase_ids.contains(&briefcase_id) {
            self.visited_briefcase_ids.push(briefcase_id);
        }
    }

    /// Gets the ID of the profile for the current step
    ///
    /// # Returns
    /// Some(profile_id) if there is a current step, None otherwise
    pub fn current_profile_id(&self) -> Option<Uuid> {
        if self.current_step_index == 0 {
            self.execution_order.first().map(|s| s.profile_id)
        } else if self.current_step_index <= self.execution_order.len() {
            self.execution_order.get(self.current_step_index - 1).map(|s| s.profile_id)
        } else {
            None
        }
    }

    /// Gets the current execution step without advancing
    ///
    /// # Returns
    /// Some(ExecutionStep) if there is a current step, None otherwise
    pub fn current_step(&self) -> Option<&ExecutionStep> {
        if self.current_step_index == 0 {
            self.execution_order.first()
        } else if self.current_step_index <= self.execution_order.len() {
            self.execution_order.get(self.current_step_index - 1)
        } else {
            None
        }
    }

    /// Resets the execution plan to the beginning
    pub fn reset(&mut self) {
        self.current_step_index = 0;
        self.visited_briefcase_ids.clear();
    }

    /// Gets total number of execution steps
    pub fn total_steps(&self) -> usize {
        self.execution_order.len()
    }

    /// Gets number of completed steps
    #[allow(dead_code)]
    pub fn completed_steps(&self) -> usize {
        self.current_step_index
    }

    /// Checks if all steps are completed
    pub fn is_complete(&self) -> bool {
        self.current_step_index >= self.execution_order.len()
    }

    /// Gets the number of visited BriefCases
    pub fn visited_count(&self) -> usize {
        self.visited_briefcase_ids.len()
    }

    /// Calculates progress as a percentage (0.0 to 1.0)
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        if self.execution_order.is_empty() {
            1.0
        } else {
            self.current_step_index as f64 / self.execution_order.len() as f64
        }
    }

    /// Gets task-specific progress (visited count / total count for each task)
    ///
    /// # Arguments
    /// * `tasks` - The task list to calculate progress for
    ///
    /// # Returns
    /// A vector of (task_index, visited, total) tuples for each task
    pub fn task_progress(&self, tasks: &[Task]) -> Vec<(usize, usize, usize)> {
        tasks
            .iter()
            .enumerate()
            .map(|(index, task)| {
                let visited_count = task
                    .related_brief_cases
                    .iter()
                    .filter(|bc| self.visited_briefcase_ids.contains(&bc.id))
                    .count();
                (index, visited_count, task.briefcase_count())
            })
            .collect()
    }
}
