use crate::commands::TaskExecutionResult;
use crate::models::task::Task;
use crate::models::{brief_case::BriefCase, profile::Profile};
use crate::project_errors::{ReadError, WriteError};
use crate::storage::Storage;
use crate::workspace::WorkspaceState;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskExecution {
    pub task_index: usize,
    pub briefcase_index: usize,
    pub profile_id: Uuid,
    pub link: String,
}

pub struct ExecutionPlan {
    pub executions: Vec<TaskExecution>,
    pub current_index: usize,
}

impl ExecutionPlan {
    fn new(tasks: &[Task]) -> Self {
        let mut profile_task_map: HashMap<Uuid, Vec<(usize, usize)>> = HashMap::new();

        for (task_idx, task) in tasks.iter().enumerate() {
            if let Some(ref brief_cases) = task.related_brief_cases {
                for (briefcase_idx, briefcase) in brief_cases.iter().enumerate() {
                    profile_task_map
                        .entry(briefcase.profile_id)
                        .or_insert_with(Vec::new)
                        .push((task_idx, briefcase_idx));
                }
            }
        }

        let mut executions = Vec::new();

        for (profile_id, task_briefcase_pairs) in profile_task_map.iter() {
            for (task_idx, briefcase_idx) in task_briefcase_pairs {
                executions.push(TaskExecution {
                    task_index: *task_idx,
                    briefcase_index: *briefcase_idx,
                    profile_id: *profile_id,
                    link: tasks[*task_idx].link.clone(),
                });
            }
        }

        ExecutionPlan {
            executions,
            current_index: 0,
        }
    }

    fn next(&mut self) -> Option<TaskExecution> {
        if self.current_index >= self.executions.len() {
            return None;
        }

        let execution = self.executions[self.current_index].clone();
        self.current_index += 1;
        Some(execution)
    }

    fn should_change_profile(&self) -> bool {
        if self.current_index >= self.executions.len() {
            return false;
        }

        if self.current_index == 0 {
            return true;
        }

        let current_profile = &self.executions[self.current_index].profile_id;
        let previous_profile = &self.executions[self.current_index - 1].profile_id;

        current_profile != previous_profile
    }

    fn reset(&mut self) {
        self.current_index = 0;
    }
}

pub struct AppState {
    pub brief_cases: RwLock<Vec<BriefCase>>,
    pub profiles: RwLock<Vec<Profile>>,
    pub workspace_state: RwLock<Option<WorkspaceState>>,
    pub execution_plan: RwLock<Option<ExecutionPlan>>,
}

impl AppState {
    pub async fn new<R: Runtime, M: Manager<R>>(manager: &M) -> Result<Self, ReadError> {
        Storage::check_files(manager).await?;
        Ok(Self {
            brief_cases: Storage::read_from_disk(manager).await?.into(),
            profiles: Storage::read_from_disk(manager).await?.into(),
            workspace_state: None.into(),
            execution_plan: None.into(),
        })
    }

    pub async fn add_brief_case(&self, brief_case: BriefCase) {
        self.brief_cases.write().await.push(brief_case);
    }

    pub async fn add_profile(&self, profile: Profile) {
        self.profiles.write().await.push(profile);
    }

    pub async fn save_profiles<R: Runtime, M: Manager<R>>(
        &self,
        app_handle: &M,
    ) -> Result<(), WriteError> {
        Storage::write_to_disk(app_handle, &self.profiles.read().await.clone()).await?;
        Ok(())
    }

    pub async fn save_brief_cases<R: Runtime, M: Manager<R>>(
        &self,
        app_handle: &M,
    ) -> Result<(), WriteError> {
        println!("Brief cases: {:?}", self.brief_cases.read().await);
        Storage::write_to_disk(app_handle, &self.brief_cases.read().await.clone()).await?;
        Ok(())
    }

    pub async fn get_profiles(&self) -> Vec<Profile> {
        self.profiles.read().await.clone()
    }

    pub async fn get_brief_cases(&self) -> Vec<BriefCase> {
        self.brief_cases.read().await.clone()
    }

    pub async fn create_workspace<R: Runtime, M: Manager<R>>(
        &self,
        manager: &M,
        tasks: Vec<Task>,
    ) -> Result<(), String> {
        let mut workspace = self.workspace_state.write().await;
        *workspace = Some(WorkspaceState::new(tasks));

        if let Some(ws) = workspace.as_mut() {
            if let Some(first_task) = ws.task_list.first() {
                if let Some(briefcases) = &first_task.related_brief_cases {
                    if let Some(first_bc) = briefcases.first() {
                        ws.set_current_profile_id(first_bc.profile_id);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn create_execution_plan(&self, tasks: &[Task]) -> Result<(), String> {
        let plan = ExecutionPlan::new(tasks);
        let mut execution_plan_lock = self.execution_plan.write().await;
        *execution_plan_lock = Some(plan);
        Ok(())
    }

    pub async fn next_execution(&self) -> Option<TaskExecution> {
        let mut execution_plan_lock = self.execution_plan.write().await;
        if let Some(plan) = execution_plan_lock.as_mut() {
            plan.next()
        } else {
            None
        }
    }

    pub async fn should_change_profile(&self) -> bool {
        let execution_plan_lock = self.execution_plan.read().await;
        if let Some(plan) = execution_plan_lock.as_ref() {
            plan.should_change_profile()
        } else {
            false
        }
    }

    pub async fn reset_execution_plan(&self) {
        let mut execution_plan_lock = self.execution_plan.write().await;
        if let Some(plan) = execution_plan_lock.as_mut() {
            plan.reset();
        }
    }

    pub async fn mark_briefcase_visited(&self, briefcase_id: uuid::Uuid) {
        let mut workspace = self.workspace_state.write().await;
        if let Some(ws) = workspace.as_mut() {
            ws.mark_briefcase_visited(briefcase_id);
        }
    }

    pub async fn get_next_briefcase_info(&self) -> Option<(usize, usize, uuid::Uuid, String)> {
        let workspace = self.workspace_state.read().await;
        if let Some(ws) = workspace.as_ref() {
            let current_task = ws.get_current_task()?;
            let current_bc = ws.get_current_briefcase()?;
            let current_profile_id = current_bc.profile_id;

            let current_task_idx = ws.current_task;

            for (task_idx, task) in ws.task_list.iter().enumerate() {
                if let Some(briefcases) = task.related_brief_cases.as_ref() {
                    for (bc_idx, bc) in briefcases.iter().enumerate() {
                        if !ws.is_briefcase_visited(bc.id)
                            && bc.profile_id == current_profile_id
                            && task_idx != current_task_idx
                        {
                            return Some((task_idx, bc_idx, bc.id, task.link.clone()));
                        }
                    }
                }
            }

            for (task_idx, task) in ws.task_list.iter().enumerate() {
                if let Some(briefcases) = task.related_brief_cases.as_ref() {
                    for (bc_idx, bc) in briefcases.iter().enumerate() {
                        if !ws.is_briefcase_visited(bc.id) && bc.profile_id != current_profile_id {
                            return Some((task_idx, bc_idx, bc.id, task.link.clone()));
                        }
                    }
                }
            }
        }
        None
    }

    pub async fn advance_to_next_briefcase(
        &self,
        app: &AppHandle,
    ) -> Result<TaskExecutionResult, String> {
        let mut workspace = self.workspace_state.write().await;
        if let Some(ws) = workspace.as_mut() {
            let current_task_idx = ws.current_task;
            let current_bc = ws.get_current_briefcase();

            let (current_bc_id, current_bc_profile_id) = if let Some(bc) = current_bc {
                let was_visited = ws.is_briefcase_visited(bc.id);
                let bc_id = bc.id;
                let bc_profile_id = bc.profile_id;

                ws.mark_briefcase_visited(bc_id);

                println!("DEBUG: Marked briefcase {:?} as visited, was_visited: {}", bc_id, was_visited);

                if let Some(task) = ws.task_list.get_mut(current_task_idx) {
                    if !was_visited {
                        task.progress += 1;
                    }
                    if task.progress >= task.comments.len() {
                        task.progress = task.comments.len() - 1;
                    }
                }

                (Some(bc_id), bc_profile_id)
            } else {
                (None, Uuid::nil())
            };

            println!("DEBUG: Current task idx: {}, current_profile_id: {:?}", current_task_idx, current_bc_profile_id);

            let target_profile_id;
            let target_link;
            let should_change;
            let target_task_idx;

            let current_task = ws.task_list.get(current_task_idx);
            let current_briefcases = current_task.and_then(|t| t.related_brief_cases.as_ref());

            if let Some(briefcases) = current_briefcases {
                for (bc_idx, bc) in briefcases.iter().enumerate() {
                    if !ws.is_briefcase_visited(bc.id) {
                        println!("DEBUG: Found next unvisited briefcase in current task: {:?}", bc.id);
                        target_profile_id = bc.profile_id;
                        target_link = ws.task_list[current_task_idx].link.clone();
                        should_change = bc.profile_id != current_bc_profile_id;
                        target_task_idx = current_task_idx;

                        ws.set_current_profile_id(target_profile_id);

                        return Ok(TaskExecutionResult {
                            profile_id: target_profile_id,
                            link: target_link,
                            should_change_profile: should_change,
                            task_index: target_task_idx,
                            completed: false,
                        });
                    }
                }
            }

            println!("DEBUG: No more briefcases in current task, searching other tasks...");
            println!("DEBUG: visited_briefcase_ids count: {}", ws.visited_briefcase_ids.len());

            for (task_idx, task) in ws.task_list.iter().enumerate() {
                if let Some(briefcases) = task.related_brief_cases.as_ref() {
                    println!("DEBUG: Task {} has {} briefcases", task_idx, briefcases.len());
                    for (bc_idx, bc) in briefcases.iter().enumerate() {
                        if !ws.is_briefcase_visited(bc.id) {
                            println!("DEBUG: Found unvisited briefcase in task {}: {:?}, profile: {:?}", task_idx, bc.id, bc.profile_id);
                            ws.current_task = task_idx;
                            target_profile_id = bc.profile_id;
                            target_link = task.link.clone();
                            should_change = true;
                            target_task_idx = task_idx;

                            ws.set_current_profile_id(target_profile_id);

                            return Ok(TaskExecutionResult {
                                profile_id: target_profile_id,
                                link: target_link,
                                should_change_profile: should_change,
                                task_index: target_task_idx,
                                completed: false,
                            });
                        }
                    }
                }
            }

            println!("DEBUG: All briefcases visited, marking complete");
            ws.is_running = false;
            return Ok(TaskExecutionResult {
                profile_id: uuid::Uuid::new_v4(),
                link: String::new(),
                should_change_profile: false,
                task_index: 0,
                completed: true,
            });
        }
        Err("No workspace state available".to_string())
    }

    pub async fn get_workspace_stats(&self) -> (usize, usize) {
        let workspace = self.workspace_state.read().await;
        if let Some(ws) = workspace.as_ref() {
            (ws.get_visited_count(), ws.get_total_briefcase_count())
        } else {
            (0, 0)
        }
    }

    pub async fn get_current_profile_window_label(&self) -> Option<String> {
        let workspace = self.workspace_state.read().await;
        workspace
            .as_ref()
            .and_then(|ws| ws.get_current_profile_label().cloned())
    }

    pub async fn set_current_profile_window_label(&self, label: String) {
        let mut workspace = self.workspace_state.write().await;
        if let Some(ws) = workspace.as_mut() {
            ws.set_current_profile_label(label);
        }
    }
}
