pub mod students;
pub mod books;

use postgres::Connection;
use rustc_serialize::{json, Encodable, Decodable};
use std::collections::HashSet;

use handlers::Optionable;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Includable {
    LentBooks,
    BaseSetBooks,
    Aliases
}

pub type Includes = HashSet<Includable>;

impl Includable {
    pub fn parse_str(val: &str) -> Includes {
        val.split(',').filter_map(|item|
            match item.to_lowercase().as_ref() {
                "aliases" => {Some(Includable::Aliases)},
                "lendings.book" | "lendings" => {Some(Includable::LentBooks)},
                "basesets.book" | "basesets" => {Some(Includable::BaseSetBooks)},
                _ => {None}
            }).collect()
    }
}

pub trait Model: Encodable + Decodable {
    fn find_id(id: usize, conn: &Connection, includes: &Includes) -> Option<Self>;
    fn find_all(conn: &Connection,includes: &Includes) -> Vec<Self>;
    fn save(self, id: Option<usize>, conn: &Connection) -> Option<Self>;
    fn delete(id: usize, conn: &Connection) -> Option<()>;

    fn parse_str(body: &str) -> Option<Self> {
        json::decode::<Self>(body).log("Deserialising (Model::parse_str)")
    }

    fn to_str(&self) -> Option<String> {
        json::encode(self).log("Serialising (Model::to_str)")
    }
}
