mod color;
extern crate r2d2;
use actix_web::{web, App, HttpServer};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
// Handler functions here
 
fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // example as reference for these two statements below :
  // -> https://docs.rs/r2d2_postgres/0.16.0/r2d2_postgres/struct.PostgresConnectionManager.html
  let manager = PostgresConnectionManager::new(
    "host=127.0.0.1 user=postgres dbname=test_db".parse().unwrap(),
    NoTls);
  let pool = r2d2::Pool::new(manager).unwrap();

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
