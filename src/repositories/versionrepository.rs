use actix_web::web;
use crate::schema::t_version::dsl::t_version;
use crate::models::versionmodel::Version;
use crate::connection::Pool;
use crate::diesel::RunQueryDsl;

pub fn get_version_list(db: web::Data<Pool>) -> Result<Vec<Version>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let records = t_version.load::<Version>(&conn)?; 
    Ok(records)
}
