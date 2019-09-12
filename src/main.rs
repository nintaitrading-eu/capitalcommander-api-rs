#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate actix_web;
extern crate diesel;
extern crate serde_derive;
extern crate dotenv;

use actix_web::{web, App, HttpServer};

pub mod api_1;
pub mod database;
pub mod models;

fn main()
{
    database::establish_connection;
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/status").route(web::get().to(api_1::status::status)))
            //.service(web::resource("/version").route(web::get().to(api_1::version::version)))
    })
    .bind("127.0.0.1:8891")
    .unwrap()
    .run()
    .unwrap();
}
