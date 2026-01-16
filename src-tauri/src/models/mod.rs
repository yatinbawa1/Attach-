pub mod brief_case;
pub mod profile;
pub mod task;

use crate::models::brief_case::BriefCase;
use crate::models::profile::Profile;

pub trait AppItem:
    serde::Serialize + serde::de::DeserializeOwned + Clone + Send + Sync + 'static
{
    fn item_type() -> &'static str;
}

impl AppItem for Profile {
    fn item_type() -> &'static str {
        "profile"
    }
}
impl AppItem for BriefCase {
    fn item_type() -> &'static str {
        "briefcase"
    }
}
