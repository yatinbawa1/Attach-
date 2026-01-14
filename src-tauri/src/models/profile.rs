use crate::models::brief_case::BriefCase;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Profile {
    profile_id: Uuid,
    profile_name: String,
    profile_path: PathBuf,
}

impl Profile {
    pub fn get_all_brief_case_ids(&self, all_brief_cases: &Vec<BriefCase>) -> Vec<Uuid> {
        all_brief_cases
            .iter()
            .filter(|brief_case| brief_case.profile_id == self.profile_id)
            .map(|brief_case| brief_case.id)
            .collect()
    }

    pub fn new(profile_name: String, profile_path: PathBuf) -> Self {
        Self {
            profile_id: Uuid::new_v4(),
            profile_name,
            profile_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::brief_case::{BriefCase, SocialMedia};
    use std::path::PathBuf;
    use uuid::Uuid;

    #[test]
    fn test_profile_new() {
        let name = String::from("Test User");
        let path = PathBuf::from("/home/user/data");

        let profile = Profile::new(name.clone(), path.clone());

        assert_eq!(profile.profile_name, name);
        assert_eq!(profile.profile_path, path);

        assert!(!profile.profile_id.is_nil());
    }

    #[test]
    fn test_get_all_brief_case_ids() {
        let profile = Profile::new("Alice".to_string(), PathBuf::from("/tmp"));
        let target_id = profile.profile_id;
        let other_id = Uuid::new_v4();

        let bf1 = BriefCase::new(
            "bf1".to_string(),
            SocialMedia::Instagram,
            target_id,
            "example1".to_string(),
        );

        let bf2 = BriefCase::new(
            "bf2".to_string(),
            SocialMedia::Instagram,
            other_id,
            "example1".to_string(),
        );

        let bf3 = BriefCase::new(
            "bf2".to_string(),
            SocialMedia::Instagram,
            target_id,
            "example1".to_string(),
        );

        let cases = vec![bf1, bf2, bf3];

        let result_ids = profile.get_all_brief_case_ids(&cases);
        
        assert_eq!(result_ids.len(), 2);
        assert_eq!(cases[0].id, result_ids[0]);
        assert_eq!(cases[2].id, result_ids[1]);
    }

    #[test]
    fn test_get_all_brief_case_ids_empty() {
        let profile = Profile::new("Empty".to_string(), PathBuf::from("/tmp"));
        let cases: Vec<BriefCase> = vec![];

        let result_ids = profile.get_all_brief_case_ids(&cases);

        assert!(result_ids.is_empty());
    }
}
