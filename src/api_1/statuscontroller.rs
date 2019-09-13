use actix_web::{HttpResponse, Responder};

pub fn status() -> impl Responder
{
    HttpResponse::Ok().json("Running...")
}
