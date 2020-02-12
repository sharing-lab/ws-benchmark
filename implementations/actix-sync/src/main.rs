mod color;
use actix_web::{web, App, HttpServer};
 

fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(actix_web::middleware::Logger::default())
      .service(web::resource("/color")
        .route(web::get().to(color::get_list)) )
  })
    .bind("127.0.0.1:55512")
    .unwrap()
    .run()
    .unwrap();
}
