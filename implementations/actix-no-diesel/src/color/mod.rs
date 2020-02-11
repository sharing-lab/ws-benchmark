pub mod common;
use actix_web::{web, HttpResponse, Error};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use futures::Future;
use postgres::NoTls;

pub fn get_list(
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> impl Future<Item = HttpResponse, Error = Error> {

  web::block(move || {

    // why mut conn ?, answer : the reference documentation is here :
    // -> https://docs.rs/r2d2_postgres/0.16.0/r2d2_postgres/struct.PostgresConnectionManager.html
    // -> ...
    // ->    let mut client = pool.get().unwrap(); 
    let mut conn = db.get().unwrap();

    let mut vec:Vec<common::Color> = Vec::new();

    for row in conn.query("SELECT id, code, name FROM color", &[]).unwrap() {
      let color = common::Color {
        id: row.get(0),
        code: row.get(1),
        name: row.get(2)
      };
      vec.push(color);
    }

    let result: Result<Vec<common::Color>, postgres::Error> = Ok(vec);
    result
  })
  .then(|res| match res {
      Ok(list) => Ok(HttpResponse::Ok().json(list)),
      Err(_) => Ok(HttpResponse::InternalServerError().into()),
  })
}
