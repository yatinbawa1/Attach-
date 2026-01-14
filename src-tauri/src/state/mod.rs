use crate::briefcase::BriefCase;
use crate::profile::Profile;
use crate::task::Task;

pub struct State {
    profiles: Vec<Profile>,
    brief_cases: Vec<BriefCase>,
    tasks: Vec<Task>,
}
