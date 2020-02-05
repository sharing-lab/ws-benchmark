use crate::schema::color;

#[table_name = "color"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Color {
    pub id: Option<i32>,
    pub code: String,
    pub name: String,
}