use serde::Serialize;
 
// This represents the payload of an HTTP Response
#[derive(Serialize)]
pub struct Color {
    pub id: i32,
    pub code: String,
    pub name: String
}
