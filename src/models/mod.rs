macro_rules! does_not_support {
    ($includable:ident, $includes:expr) => (try!(
        if $includes.contains(&$crate::models::Includable::$includable) {
            ::std::result::Result::Err($crate::error::ObsidianError::IncludeNotSupported($crate::models::Includable::$includable))
        } else {
            Ok(())
        }
    ));
}

pub mod students;
pub mod books;
pub mod aliases;
pub mod teachers;
pub mod base_sets;
pub mod lendings;
pub mod schools;
pub mod sessions;

use postgres::Connection;
use rustc_serialize::{json, Encodable, Decodable};
use std::collections::HashSet;

use error::ObsidianError;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
    fn find_id(id: usize, school_id: usize, conn: &Connection, includes: &Includes) -> Result<Self, ObsidianError>;
    fn find_all(school_id: usize, conn: &Connection,includes: &Includes) -> Result<Vec<Self>, ObsidianError>;
    fn save(self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError>;
    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError>;

    fn parse_str(body: &str) -> Result<Self, ObsidianError> {
        json::decode::<Self>(body).map_err(ObsidianError::from)
    }

    fn to_str(&self) -> Result<String, ObsidianError> {
        json::encode(self).map_err(ObsidianError::from)
    }
}
