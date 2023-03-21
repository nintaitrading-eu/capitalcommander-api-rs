use actix_web::{web, HttpResponse, Error};

use crate::repositories::accountrepository;
use crate::connection::Pool;

pub async fn get_account_list(db: web::Data<Pool>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || accountrepository::get_account_list(db))
       .await
       .map(|account| HttpResponse::Ok().json(account))
       .map_err(|_| HttpResponse::InternalServerError())?)
}


pub async fn get_account_by_id(
    db: web::Data<Pool>,
    id: web::Path<i64>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || accountrepository::get_account_by_id(db, id.into_inner()))
       .await
       .map(|account| HttpResponse::Ok().json(account))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_account() -> HttpResponse
{
    HttpResponse::Ok().json("Not implemented yet...")
}

pub async fn delete_account() -> HttpResponse
{
    HttpResponse::Ok().json("Not implemented yet...")
}
