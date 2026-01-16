mod storage_test {
    use super::*;
    use crate::models::profile::Profile;
    use crate::storage::*;
    use std::path::PathBuf;
    use std::vec;

    #[cfg(test)]
    #[tokio::test]
    async fn initialize_storage() {
        let storage: Storage = Storage::new().await.unwrap();

        assert_eq!(
            storage.config_file.profile_path,
            PathBuf::from("/Users/yatin/Library/Application Support/attache/profile.json")
        );
        assert_eq!(storage.current_profiles, None);
        assert_eq!(storage.current_brief_cases, None);
    }

    #[tokio::test]
    async fn test_load_brief_case_empty() {
        let mut storage: Storage = Storage::new().await.unwrap();
        storage.read_brief_cases_from_disk().await.unwrap();
        println!("{:?}", storage.current_brief_cases);
    }

    #[tokio::test]
    async fn test_load_profile_empty() {
        let mut storage: Storage = Storage::new().await.unwrap();
        storage.read_profiles_from_disk_into_current().await.unwrap();
        println!("{:?}", storage.current_profiles);
    }

    #[tokio::test]
    async fn test_write_profiles() {
        let mut storage: Storage = Storage::new().await.unwrap();
	    let mut profile_array: Vec<Profile> = Vec::new();

	    for i in 0..10{
		    profile_array.push(Profile::new(format!("Example {i}").to_string(), PathBuf::from(format!("example{i}"))));
	    }

        storage.write_profiles_to_disk(&profile_array).await.unwrap();
	    let new_storage = storage.read_profiles_from_disk_into_current().await.unwrap();

    }
}
