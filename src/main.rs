pub mod schema;
pub mod connection;
pub mod errors;
pub mod dto;
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

use actix_web::{web, App, HttpServer};
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
            .route("/account/{id}", web::post().to(api_1::accountcontroller::update_account))
            .route("/account/{id}", web::delete().to(api_1::accountcontroller::delete_account))
            .route("/trade", web::get().to(api_1::tradecontroller::get_trade_list))
            .route("/trade/{id}", web::get().to(api_1::tradecontroller::get_trade_by_id))
            .route("/trade", web::post().to(api_1::tradecontroller::add_trade))
            .route("/trade/{id}", web::post().to(api_1::tradecontroller::update_trade))
            .route("/trade/{id}", web::delete().to(api_1::tradecontroller::delete_trade))
    })
    .bind("127.0.0.1:8891")?
    .run()
    .await
}
