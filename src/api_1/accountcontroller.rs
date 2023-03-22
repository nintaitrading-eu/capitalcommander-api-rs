use actix_web::{web, HttpResponse, Error};

use crate::dto::accountdto::AccountDto;
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

pub async fn add_account(
    db: web::Data<Pool>,
    item: web::Json<AccountDto>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || accountrepository::add_account(db, item))
       .await
       .map(|account| HttpResponse::Created().json(account))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_account(
    db: web::Data<Pool>,
    id: web::Path<i64>,
    item: web::Json<AccountDto>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || accountrepository::update_account(db, id.into_inner(), item))
       .await
       .map(|account| HttpResponse::Ok().json(account))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_account(
    db: web::Data<Pool>,
    id: web::Path<i64>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || accountrepository::delete_account(db, id.into_inner()))
       .await
       .map(|account| HttpResponse::Ok().json(account))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
