mod storage_test {
    use crate::models::profile::Profile;
    use crate::storage::*;
    use std::path::PathBuf;
    use crate::models::brief_case::BriefCase;

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
        let storage: Storage = Storage::new().await.unwrap();
        let cases: Vec<Profile> = storage.read_from_disk().await.unwrap();
        println!("{:?}", cases);
    }

    #[tokio::test]
    async fn test_load_profile_empty() {
        let storage: Storage = Storage::new().await.unwrap();
        let profiles: Vec<BriefCase> = storage.read_from_disk().await.unwrap();
        println!("{:?}", profiles);
    }

    #[tokio::test]
    async fn test_write() {
        let storage: Storage = Storage::new().await.unwrap();
        let mut profile_array: Vec<Profile> = Vec::new();

        for i in 0..10 {
            profile_array.push(Profile::new(
                format!("Example {i}").to_string(),
                PathBuf::from(format!("example{i}")),
            ));
        }

        storage.write_to_disk(&profile_array).await.unwrap();
        let profiles: Vec<Profile> = storage.read_from_disk().await.unwrap();
    }
}
