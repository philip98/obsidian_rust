use postgres::Connection;
use postgres::rows::Row;
use std::collections::HashSet;

use error::ObsidianError;
use models::{Model, Includes};
use models::books::Book;

const QUERY_ALIASES: &'static str = "SELECT aliases.id, book_id, name FROM aliases, books WHERE book_id = books.id
    AND school_id = $1";

const INSERT_ALIAS: &'static str = "INSERT INTO aliases (book_id, name) VALUES ($1, $2) RETURNING id";
const UPDATE_ALIAS: &'static str = "UPDATE aliases SET book_id=$2, name=$3 WHERE aliases.id=$1 AND
EXISTS (SELECT * FROM books WHERE books.id = aliases.book_id AND books.school_id = $4)";
const DELETE_ALIAS: &'static str = "DELETE FROM aliases WHERE aliases.id=$1 AND EXISTS
(SELECT * FROM books WHERE books.id = aliases.book_id AND books.school_id = $2)";

#[derive(RustcEncodable, RustcDecodable)]
pub struct Alias {
    id: Option<usize>,
    book_id: usize,
    name: String
}

impl Alias {
    pub fn new(id: Option<usize>, book_id: usize, name: String) -> Alias {
        Alias {
            id: id,
            book_id: book_id,
            name: name
        }
    }

    fn from_db(_: &Connection, _: &Includes, row: Row) -> Alias {
        Alias {
            id: Some(row.get::<usize, i32>(0) as usize),
            book_id: row.get::<usize, i32>(1) as usize,
            name: row.get::<usize, String>(2) as String
        }
    }
}

impl Model for Alias {
    fn find_id(_: usize, _: usize, _: &Connection, _: &Includes) -> Result<Self, ObsidianError> {
        unreachable!()
    }

    fn find_all(school_id: usize, conn: &Connection, includes: &Includes) -> Result<Vec<Self>, ObsidianError> {
        does_not_support!(LentBooks, includes);
        does_not_support!(BaseSetBooks, includes);
        does_not_support!(Aliases, includes);
        let stmt = try!(conn.prepare_cached(QUERY_ALIASES));
        let rows = try!(stmt.query(&[&(school_id as i32)]))
            .iter()
            .map(|row| Alias::from_db(conn, includes, row))
            .collect::<Vec<Self>>();
        Ok(rows)
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError> {
        if let Some(id) = id {
            let stmt = try!(conn.prepare_cached(UPDATE_ALIAS));
            let modified = try!(stmt.execute(&[&(id as i32), &(self.book_id as i32),
                &self.name, &(school_id as i32)]));
            if modified == 1 {
                self.id = Some(id);
                Ok(self)
            } else {
                Err(ObsidianError::RecordNotFound("Alias"))
            }
        } else {
            try!(Book::find_id(self.book_id, school_id, conn, &HashSet::new()));
            let stmt = try!(conn.prepare_cached(INSERT_ALIAS));
            let rows = try!(stmt.query(&[&(self.book_id as i32), &self.name]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_ALIAS));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("Alias"))
        }
    }
}
