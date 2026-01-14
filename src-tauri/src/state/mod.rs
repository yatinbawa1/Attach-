use crate::models::brief_case::BriefCase;
use crate::models::profile::Profile;
use crate::models::task::Task;

pub struct State {
    profiles: Vec<Profile>,
    brief_cases: Vec<BriefCase>,
    tasks: Vec<Task>,
}
