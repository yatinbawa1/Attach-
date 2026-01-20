use crate::execution::ExecutionPlan;
use crate::models::{BriefCase, Profile, Task};
use tokio::sync::RwLock;

/// Global application state managing all data and execution progress
///
/// AppState is the central point of the application that:
/// 1. Stores all Profiles, BriefCases, and Tasks
/// 2. Manages the current execution plan and progress
/// 3. Coordinates between the UI and backend execution
/// 4. Handles persistence to disk
pub struct AppState {
    /// All browser profiles in the system
    profiles: RwLock<Vec<Profile>>,
    /// All social media user accounts in the system
    brief_cases: RwLock<Vec<BriefCase>>,
    /// Current tasks being executed
    tasks: RwLock<Vec<Task>>,
    /// The execution plan for the current session
    execution_plan: RwLock<Option<ExecutionPlan>>,
    /// Label of the currently active profile window
    current_window_label: RwLock<Option<String>>,
    /// Whether automation is currently running
    is_running: RwLock<bool>,
}

impl AppState {
    /// Creates a new AppState instance and loads data from disk
    ///
    /// # Arguments
    /// * `profiles` - Initial list of profiles
    /// * `brief_cases` - Initial list of briefcases
    ///
    /// # Returns
    /// A new AppState instance
    pub fn new(profiles: Vec<Profile>, brief_cases: Vec<BriefCase>) -> Self {
        Self {
            profiles: RwLock::new(profiles),
            brief_cases: RwLock::new(brief_cases),
            tasks: RwLock::new(Vec::new()),
            execution_plan: RwLock::new(None),
            current_window_label: RwLock::new(None),
            is_running: RwLock::new(false),
        }
    }

    // ==================== Profile Management ====================

    /// Adds a new profile to the state
    pub async fn add_profile(&self, profile: Profile) {
        self.profiles.write().await.push(profile);
    }

    /// Gets all profiles
    pub async fn get_profiles(&self) -> Vec<Profile> {
        self.profiles.read().await.clone()
    }

    /// Sets the profiles list
    pub async fn set_profiles(&self, profiles: Vec<Profile>) {
        *self.profiles.write().await = profiles;
    }

    /// Finds a profile by ID
    pub async fn get_profile_by_id(&self, profile_id: uuid::Uuid) -> Option<Profile> {
        self.profiles
            .read()
            .await
            .iter()
            .find(|p| p.profile_id == profile_id)
            .cloned()
    }

    // ==================== BriefCase Management ====================

    /// Adds a new BriefCase to the state
    pub async fn add_brief_case(&self, brief_case: BriefCase) {
        self.brief_cases.write().await.push(brief_case);
    }

    /// Gets all BriefCases
    pub async fn get_brief_cases(&self) -> Vec<BriefCase> {
        self.brief_cases.read().await.clone()
    }

    /// Sets the BriefCases list
    pub async fn set_brief_cases(&self, brief_cases: Vec<BriefCase>) {
        *self.brief_cases.write().await = brief_cases;
    }

    // ==================== Task Management ====================

    /// Sets the tasks for the current session and creates an execution plan
    pub async fn set_tasks(&self, tasks: Vec<Task>) {
        let plan = ExecutionPlan::new(&tasks);
        *self.tasks.write().await = tasks;
        *self.execution_plan.write().await = Some(plan);
    }

    /// Gets all tasks
    pub async fn get_tasks(&self) -> Vec<Task> {
        self.tasks.read().await.clone()
    }

    /// Gets a task by index
    pub async fn get_task(&self, index: usize) -> Option<Task> {
        self.tasks.read().await.get(index).cloned()
    }

    /// Sets the comment index for a task
    pub async fn set_task_comment_index(&self, task_index: usize, comment_index: usize) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_index) {
            if comment_index < task.comments.len() {
                task.comment_index = comment_index;
            }
        }
    }

    /// Gets the total number of tasks
    pub async fn task_count(&self) -> usize {
        self.tasks.read().await.len()
    }

    /// Gets the current comment for a task
    pub async fn get_current_comment(&self, task_index: usize) -> Option<String> {
        let tasks = self.tasks.read().await;
        if let Some(task) = tasks.get(task_index) {
            if task.comments.is_empty() {
                None
            } else {
                let index = task.comment_index % task.comments.len();
                Some(task.comments.get(index).cloned().unwrap_or_default())
            }
        } else {
            None
        }
    }

    /// Increments the comment index for a task (with wrapping)
    pub async fn increment_comment_index(&self, task_index: usize) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_index) {
            if !task.comments.is_empty() {
                task.comment_index = (task.comment_index + 1) % task.comments.len();
            }
        }
    }

    // ==================== Execution Plan Management ====================

    /// Creates a new execution plan from the current tasks
    pub async fn create_execution_plan(&self) {
        let tasks = self.tasks.read().await.clone();
        let plan = ExecutionPlan::new(&tasks);
        *self.execution_plan.write().await = Some(plan);
    }

    /// Gets the next execution step
    pub async fn next_execution_step(&self) -> Option<crate::execution::ExecutionStep> {
        let mut plan_lock = self.execution_plan.write().await;
        if let Some(ref mut plan) = *plan_lock {
            plan.next()
        } else {
            None
        }
    }

    /// Checks if the next step requires a profile change
    pub async fn should_change_profile(&self) -> bool {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.should_change_profile()
        } else {
            false
        }
    }

    /// Gets the current profile ID from the execution plan
    pub async fn current_profile_id(&self) -> Option<uuid::Uuid> {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.current_profile_id()
        } else {
            None
        }
    }

    /// Gets the current execution step without advancing
    pub async fn current_step(&self) -> Option<crate::execution::ExecutionStep> {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.current_step().cloned()
        } else {
            None
        }
    }

    /// Marks a BriefCase as visited
    pub async fn mark_briefcase_visited(&self, briefcase_id: uuid::Uuid) {
        let mut plan_lock = self.execution_plan.write().await;
        if let Some(ref mut plan) = *plan_lock {
            plan.mark_visited(briefcase_id);
        }
    }

    /// Resets the execution plan
    pub async fn reset_execution_plan(&self) {
        let mut plan_lock = self.execution_plan.write().await;
        if let Some(ref mut plan) = *plan_lock {
            plan.reset();
        }
    }

    /// Checks if the execution plan is complete
    pub async fn is_complete(&self) -> bool {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.is_complete()
        } else {
            false
        }
    }

    // ==================== Progress Tracking ====================

    /// Gets overall progress statistics
    ///
    /// # Returns
    /// A tuple of (visited_count, total_count)
    pub async fn get_progress(&self) -> (usize, usize) {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            (plan.visited_count(), plan.total_steps())
        } else {
            (0, 0)
        }
    }

    /// Gets task-specific progress for all tasks
    ///
    /// # Returns
    /// A vector of (task_index, visited_count, total_count) tuples
    pub async fn get_task_progress(&self) -> Vec<(usize, usize, usize)> {
        let plan_lock = self.execution_plan.read().await;
        let tasks_lock = self.tasks.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.task_progress(&tasks_lock)
        } else {
            Vec::new()
        }
    }

    /// Gets the current task index
    pub async fn current_task_index(&self) -> Option<usize> {
        let plan_lock = self.execution_plan.read().await;
        if let Some(ref plan) = *plan_lock {
            plan.current_step().map(|s| s.task_index)
        } else {
            None
        }
    }

    // ==================== Window Management ====================

    /// Sets the label of the currently active profile window
    pub async fn set_current_window_label(&self, label: String) {
        *self.current_window_label.write().await = Some(label);
    }

    /// Gets the label of the currently active profile window
    pub async fn get_current_window_label(&self) -> Option<String> {
        self.current_window_label.read().await.clone()
    }

    /// Clears the current window label
    pub async fn clear_current_window_label(&self) {
        *self.current_window_label.write().await = None;
    }

    // ==================== Running State ====================

    /// Sets whether automation is running
    pub async fn set_running(&self, running: bool) {
        *self.is_running.write().await = running;
    }

    /// Checks if automation is currently running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }
}
