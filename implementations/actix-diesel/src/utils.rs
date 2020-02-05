use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct WebServiceResponse {
    pub status: String,
    pub info: String,
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
