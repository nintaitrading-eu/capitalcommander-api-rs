use actix_web::{HttpResponse, Responder};

pub fn version() -> impl Responder
{

    HttpResponse::Ok().json("Version info:")
}
