use actix_web::{web, HttpResponse, Error};

use crate::dto::tradedto::TradeDto;
use crate::repositories::traderepository;
use crate::connection::Pool;

pub async fn get_trade_list(db: web::Data<Pool>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || traderepository::get_trade_list(db))
       .await
       .map(|trade| HttpResponse::Ok().json(trade))
       .map_err(|_| HttpResponse::InternalServerError())?)
}


pub async fn get_trade_by_id(
    db: web::Data<Pool>,
    id: web::Path<i64>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || traderepository::get_trade_by_id(db, id.into_inner()))
       .await
       .map(|trade| HttpResponse::Ok().json(trade))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_trade(
    db: web::Data<Pool>,
    item: web::Json<TradeDto>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || traderepository::add_trade(db, item))
       .await
       .map(|trade| HttpResponse::Created().json(trade))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_trade(
    db: web::Data<Pool>,
    id: web::Path<i64>,
    item: web::Json<TradeDto>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || traderepository::update_trade(db, id.into_inner(), item))
       .await
       .map(|trade| HttpResponse::Ok().json(trade))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_trade(
    db: web::Data<Pool>,
    id: web::Path<i64>) -> Result<HttpResponse, Error>
{
    Ok(web::block(move || traderepository::delete_trade(db, id.into_inner()))
       .await
       .map(|trade| HttpResponse::Ok().json(trade))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
