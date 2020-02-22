//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpServer};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
use dotenv;

mod models;
mod schema;
#[macro_use]
mod utils;
mod service;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new("postgres://postgres@127.0.0.1/test_db");
    let pool = Pool::builder().max_size(100).build(manager).unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(actix_web::middleware::Logger::default())
            .service( web::resource("/color")
                .route(web::get().to_async(service::list_color)) )
    })
    .bind("127.0.0.1:55502")?
    .run()
}
