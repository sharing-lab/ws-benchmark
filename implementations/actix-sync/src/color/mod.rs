pub mod common;
extern crate postgres;
use postgres::{Client, NoTls};
use actix_web::{Responder, web};

static PG_CONNECTION_STRING: &str = "host=127.0.0.1 user=postgres dbname=test_db";

pub fn get_list() -> impl Responder {
  let mut vec:Vec<common::Color> = Vec::new();

  let mut conn = Client::connect(PG_CONNECTION_STRING, NoTls) .unwrap();
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
