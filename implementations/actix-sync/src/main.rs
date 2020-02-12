mod color;
use actix_web::{web, App, HttpServer, Responder};
 
// Handler functions here
 
fn main() {
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/color")
        .route(web::get().to(color::get_list)) )
  })
    .bind("127.0.0.1:55512")
    .unwrap()
    .run()
    .unwrap();
}
