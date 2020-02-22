#![allow(dead_code)]
use std::{cmp, io};

use bytes::{BufMut, BytesMut};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct Color {
    pub id: i32,
    pub code: String,
    pub name: String,
}

pub const SIZE: usize = 27;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: &'static str,
}

pub struct Writer<'a>(pub &'a mut BytesMut);

impl<'a> io::Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.put_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn get_query_param(query: &str) -> u16 {
    let q = if let Some(pos) = query.find("q") {
        query.split_at(pos + 2).1.parse::<u16>().ok().unwrap_or(1)
    } else {
        1
    };
    cmp::min(500, cmp::max(1, q))
}

