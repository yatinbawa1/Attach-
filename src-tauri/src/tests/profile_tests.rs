use super::*;
use crate::models::brief_case::{BriefCase, SocialMedia};

use crate::models::profile::Profile;
use std::path::PathBuf;
use uuid::Uuid;

#[cfg(test)]
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
