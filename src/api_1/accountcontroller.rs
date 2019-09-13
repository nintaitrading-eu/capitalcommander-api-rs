use actix_web::{HttpRequest, HttpResponse, web};

use crate::models::accountmodel::AccountList;
use crate::models::accountmodel::AccountNew;

pub fn index(_req: HttpRequest) -> HttpResponse
{
    HttpResponse::Ok().json(AccountList::list())
}

pub fn create(new_product: web::Json<AccountNew>) -> Result<HttpResponse, HttpResponse>
{
    new_product
      .create()
      .map(|account| HttpResponse::Ok().json(account))
      .map_err(|e|  {
        HttpResponse::InternalServerError().json(e.to_string())
      })
}
