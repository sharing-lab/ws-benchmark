use std::collections::HashMap;
use std::fmt::Write;
use std::io;

use actix::prelude::*;
use bytes::{Bytes, BytesMut};
use futures::stream::futures_unordered::FuturesUnordered;
use futures::{FutureExt, StreamExt, TryStreamExt};
use tokio_postgres::types::ToSql;
use tokio_postgres::{connect, Client, NoTls, Statement};

use crate::utils::{Color, Writer};

/// Postgres interface
pub struct PgConnection {
    cl: Client,
    colors: Statement
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: &str) -> Result<Addr<PgConnection>, io::Error> {
        let (cl, conn) = connect(db_url, NoTls)
            .await
            .expect("can not connect to postgresql");
        actix_rt::spawn(conn.map(|_| ()));

        let colors = cl.prepare("SELECT id, code, name FROM color").await.unwrap();
        Ok(PgConnection::create(move |_| PgConnection {
            cl, colors
        }))
    }
}

pub struct Colors;

impl Message for Colors {
    type Result = io::Result<Vec<Color>>;
}

impl Handler<Colors> for PgConnection {
    type Result = ResponseFuture<Result<Vec<Color>, io::Error>>;

    fn handle(&mut self, _: Colors, _: &mut Self::Context) -> Self::Result {
        let mut items: Vec<Color> = Vec::new();
        let fut = self.cl.query_raw(&self.colors, &[]);

        Box::pin(async move {
            let mut stream = fut
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

            while let Some(row) = stream.next().await {
                let row = row.map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("{:?}", e))
                })?;
                items.push(Color {
                    id: row.get(0),
                    code: row.get(1),
                    name: row.get(2),
                });
            }

            Ok(items)
        })
    }
}
