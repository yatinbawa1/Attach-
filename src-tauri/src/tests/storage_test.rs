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
    }

    #[tokio::test]
    async fn test_load_brief_case_empty() {
        let mut storage: Storage = Storage::new().await.unwrap();
        let cases = storage.read_brief_cases_from_disk().await.unwrap();
        println!("{:?}", cases);
    }

    #[tokio::test]
    async fn test_load_profile_empty() {
        let mut storage: Storage = Storage::new().await.unwrap();
        let profiles =  storage.read_profiles_from_disk().await.unwrap();
        println!("{:?}", profiles);
    }

    #[tokio::test]
    async fn test_write_profiles() {
        let mut storage: Storage = Storage::new().await.unwrap();
	    let mut profile_array: Vec<Profile> = Vec::new();

	    for i in 0..10{
		    profile_array.push(Profile::new(format!("Example {i}").to_string(), PathBuf::from(format!("example{i}"))));
	    }

        storage.write_profiles_to_disk(&profile_array).await.unwrap();
	    let profiles = storage.read_profiles_from_disk().await.unwrap();
    }
}
