use actix_web::HttpResponse;

use crate::models::versionmodel::VersionList;

pub async fn get_version() -> HttpResponse
{
    HttpResponse::Ok().json(VersionList::list())
}
