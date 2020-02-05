use actix_web::{web, HttpResponse, Error};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use futures::Future;

use crate::models;
use crate::schema;
use crate::utils;


/// Async request handler
pub fn list_color(
    pool: web::Data<utils::PgPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || -> Result<Vec<models::Color>, diesel::result::Error> {
        use self::schema::color::dsl::*;
        let conn: &PgConnection = &pool.get().unwrap();
        let items = color.load::<models::Color>(conn);
        items
    })
    .then(|res| match res {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
