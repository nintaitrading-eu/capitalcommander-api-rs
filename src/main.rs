pub mod schema;
pub mod connection;
pub mod models;
pub mod repositories;
pub mod api_1;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix_web;
extern crate actix_rt;
extern crate futures;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use crate::connection as dbconn;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Note: to_async not working... check later why
    HttpServer::new(move || {
        App::new()
            .data(dbconn::establish_connection().clone())
            .route("/status", web::get().to(api_1::statuscontroller::status))
            .route("/version", web::get().to(api_1::versioncontroller::get_version))
            .route("/account", web::get().to(api_1::accountcontroller::get_account_list))
            .route("/account/{id}", web::get().to(api_1::accountcontroller::get_account_by_id))
            .route("/account", web::post().to(api_1::accountcontroller::add_account))
            .route("/account/{id}", web::delete().to(api_1::accountcontroller::delete_account))
    })
    .bind("127.0.0.1:8891")?
    .run()
    .await
}
