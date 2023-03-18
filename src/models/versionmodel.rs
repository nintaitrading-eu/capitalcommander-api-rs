use crate::schema::t_version;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct VersionList(pub Vec<Version>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Version
{
    pub version_id: i32,
    pub database_version: String,
    pub database_version_info: String,
    pub application_version: String,
    pub application_version_info: String,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}
