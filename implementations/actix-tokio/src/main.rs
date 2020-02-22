#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_http::{HttpService, KeepAlive};
use actix_service::map_config;
use actix_web::dev::{AppConfig, Body, Server};
use actix_web::http::{header::CONTENT_TYPE, header::SERVER, HeaderValue, StatusCode};
use actix_web::{web, App, Error, HttpResponse};
use bytes::BytesMut;

mod utils;
mod db;
use crate::db::{PgConnection, Colors};
use crate::utils::{Writer};

async fn list_color(db: web::Data<Addr<PgConnection>>) -> Result<HttpResponse, Error> {
    let res = db.send(Colors).await?;
    if let Ok(worlds) = res {
        let mut body = BytesMut::with_capacity(35 * worlds.len());
        serde_json::to_writer(Writer(&mut body), &worlds).unwrap();
        let mut res =
            HttpResponse::with_body(StatusCode::OK, Body::Bytes(body.freeze()));
        res.headers_mut()
            .insert(SERVER, HeaderValue::from_static("Actix"));
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(res)
    } else {
        Ok(HttpResponse::InternalServerError().into())
    }}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Started http server: 127.0.0.1:55519");

    const DB_URL: &str =
        "postgres://postgres:@127.0.0.1/test_db";

    // start http server
    Server::build()
        .backlog(1024)
        .bind("web-server", "0.0.0.0:55519", move || {
            HttpService::build()
                .keep_alive(KeepAlive::Os)
                .h1(map_config(
                    App::new()
                        .data_factory(|| PgConnection::connect(DB_URL))
                        // enable logger
                        .wrap(actix_web::middleware::Logger::default())
                        .service(web::resource("/color").to(list_color)),
                    |_| AppConfig::default(),
                ))
                .tcp()
        })?
        .start()
        .await
}
