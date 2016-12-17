pub mod students;

use postgres::Connection;
use rustc_serialize::{json, Encodable, Decodable};

pub trait Model: Encodable + Decodable {
    fn find_id(id: usize, conn: &Connection) -> Option<Self>;
    fn find_all(conn: &Connection) -> Vec<Self>;
    fn save(self, id: Option<usize>, conn: &Connection) -> Option<Self>;
    fn delete(id: usize, conn: &Connection) -> Option<()>;

    fn parse_str(body: &str) -> Option<Self> {
        json::decode::<Self>(body).ok()
    }

    fn to_str(&self) -> Option<String> {
        json::encode(self).ok()
    }
}
