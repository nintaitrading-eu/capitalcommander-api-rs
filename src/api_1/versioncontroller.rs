use actix_web::{HttpRequest, HttpResponse};

use crate::models::versionmodel::VersionList;

pub fn index(_req: HttpRequest) -> HttpResponse
{
    HttpResponse::Ok().json(VersionList::list())
}
