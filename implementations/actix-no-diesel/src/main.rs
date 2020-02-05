mod color;
extern crate r2d2;
use actix_web::{web, App, HttpServer};
use r2d2_postgres::{r2d2::Pool, PostgresConnectionManager, TlsMode};
// Handler functions here
 
fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let manager = PostgresConnectionManager::new(
    "postgres://postgres@localhost/test_db",
    TlsMode::None,
    )
    .unwrap();
  let pool = Pool::builder().build(manager).unwrap();

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      // enable logger
      .wrap(actix_web::middleware::Logger::default())
      .service(web::resource("/color")
        .route(web::get().to_async(color::get_list)) )
  })
    .bind("127.0.0.1:55507")
    .unwrap()
    .run()
    .unwrap();
}
