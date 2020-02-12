pub mod common;
extern crate postgres;
use postgres::{Client, NoTls};
use actix_web::{Responder, web};

static pg_connection_string: &str = "host=127.0.0.1 user=postgres dbname=test_db";

// pub fn get_kapal_all(vec: &mut Vec<common::Kapal>) {
//   let conn = Connection::connect(pg_connection_string, TlsMode::None) .unwrap();
//   for row in &conn.query("SELECT id, kode FROM kapal", &[]).unwrap() {
//     let kapal = common::Kapal {
//       id: row.get(0),
//       kode: row.get(1)
//     };
//     vec.push(kapal);
//   }
// }

pub fn get_list() -> impl Responder {
  let mut vec:Vec<common::Color> = Vec::new();

//  kapal::get_kapal_all(&mut vec);
  let mut conn = Client::connect(pg_connection_string, NoTls) .unwrap();
  for row in conn.query("SELECT id, code, name FROM color", &[]).unwrap() {
    let color = common::Color {
      id: row.get(0),
      code: row.get(1),
      name: row.get(2)
    };
    vec.push(color);
  }

  return web::Json(vec);
}
