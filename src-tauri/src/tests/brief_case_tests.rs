mod brief_case_test {
    use crate::models::brief_case;
    use crate::models::brief_case::{BriefCase, SocialMedia};
    use uuid::Uuid;

    #[cfg(test)]
    #[test]
    fn test_brief_case_new() {
        let bf1 = BriefCase::new(
            "bf1".to_string(),
            SocialMedia::Instagram,
            Uuid::new_v4(),
            "example1".to_string(),
        );

        assert_eq!(bf1.user_name, "example1");
        assert_eq!(bf1.name, "bf1");
        assert_eq!(bf1.platform, SocialMedia::Instagram);
    }
}
