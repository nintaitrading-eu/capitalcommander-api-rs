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

impl VersionList
{
    pub fn list() -> Self
    {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::t_version::dsl::*;
        use crate::connection::establish_connection;

        let connection = establish_connection();

        let result =
            t_version
                .limit(1)
                .load::<Version>(&connection)
                .expect("Error loading version");

        VersionList(result)
    }
}
