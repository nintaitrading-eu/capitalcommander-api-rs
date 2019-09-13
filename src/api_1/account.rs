use actix_web::{HttpRequest, HttpResponse};

use crate::models::accountmodel::AccountList;

pub fn account(_req: HttpRequest) -> HttpResponse
{
    HttpResponse::Ok().json(AccountList::list())
}
