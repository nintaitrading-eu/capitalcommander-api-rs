pub mod schema;
pub mod connection;
pub mod models;
pub mod api_1;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{web, App, HttpServer};


fn main()
{
    let sys = actix::System::new("CapitalCommander");

    // Note: to_async not working... check later why
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/status")
                .route(web::get().to(api_1::statuscontroller::status)))
            .service(web::resource("/version")
                .route(web::get().to(api_1::versioncontroller::index)))
            .service(web::resource("/account")
                .route(web::get().to(api_1::accountcontroller::index)))
    })
    .bind("127.0.0.1:8891")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8891");
    let _ = sys.run();
}
