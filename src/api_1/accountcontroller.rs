use actix_web::{HttpRequest, HttpResponse};

use crate::models::accountmodel::AccountList;

pub fn index(_req: HttpRequest) -> HttpResponse
{
    HttpResponse::Ok().json(AccountList::list())
}
