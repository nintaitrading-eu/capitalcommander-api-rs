use actix_web::HttpResponse;

use crate::models::accountmodel::AccountList;
use crate::models::accountmodel::AccountNew;

pub async fn get_account_list() -> HttpResponse
{
    HttpResponse::Ok().json(AccountList::list())
}

pub async fn get_account_by_id() -> HttpResponse
{
    HttpResponse::Ok().json("Not implemented yet...")
}

pub async fn add_account() -> HttpResponse
{
    HttpResponse::Ok().json("Not implemented yet...")
}

pub async fn delete_account() -> HttpResponse
{
    HttpResponse::Ok().json("Not implemented yet...")
}
