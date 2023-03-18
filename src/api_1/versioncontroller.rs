use actix_web::{web, HttpResponse, Error};

use crate::repositories::versionrepository;
use crate::connection::Pool;

pub async fn get_version(db: web::Data<Pool>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || versionrepository::get_version_list(db))
       .await
       .map(|version| HttpResponse::Ok().json(version))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
