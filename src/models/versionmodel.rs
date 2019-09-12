use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Version
{
    pub version_id: i32,
    pub database_version: String,
    pub database_version_info: String,
    pub application_version: String,
    pub application_version_info: String,
}
