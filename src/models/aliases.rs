use postgres::Connection;
use postgres::rows::Row;

use models::{Model, Includes};
use handlers::Optionable;

const QUERY_ALIAS: &'static str = "SELECT id, book_id, name FROM aliases WHERE id=$1";
const QUERY_ALIASES: &'static str = "SELECT id, book_id, name FROM aliases";

const INSERT_ALIAS: &'static str = "INSERT INTO aliases (book_id, name) VALUES ($1, $2) RETURNING id";
const UPDATE_ALIAS: &'static str = "UPDATE aliases SET book_id=$2, name=$3 WHERE id=$1";
const DELETE_ALIAS: &'static str = "DELETE FROM aliases WHERE id=$1";

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
    fn find_id(id: usize, conn: &Connection, includes: &Includes) -> Option<Self> {
        if !includes.is_empty() {
            None.log("Include param not supported")
        } else {
            conn.prepare_cached(QUERY_ALIAS).log("Preparing SELECT aliases query (Alias::find_id)")
                .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT aliases query (Alias::find_id)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| Alias::from_db(conn, includes, row))
                        .log("Row not found (Alias::find_id)")))
                .and_then(|alias| if alias.id == Some(id) {Some(alias)} else {None}
                    .log("IDs don't match (Alias::find_id)"))
        }
    }

    fn find_all(conn: &Connection, includes: &Includes) -> Vec<Self> {
        if !includes.is_empty() {
            vec![]
        } else {
            conn.prepare_cached(QUERY_ALIASES).log("Preparing SELECT aliases query (Alias::find_all)")
                .and_then(|stmt| stmt.query(&[]).log("Executing SELECT aliases query (Alias::find_all)")
                    .map(|rows| rows
                        .iter()
                        .map(|row| Alias::from_db(conn, includes, row))
                        .collect::<Vec<Alias>>()))
                .unwrap_or(vec![])
        }
    }

    fn save(mut self, id: Option<usize>, conn: &Connection) -> Option<Self> {
        if let Some(id) = id {
            conn.prepare_cached(UPDATE_ALIAS).log("Preparing UPDATE aliases query (Alias::save)")
                .and_then(|stmt| stmt.execute(&[&(id as i32), &(self.book_id as i32), &self.name])
                    .log("Executing UPDATE aliases query (Alias::save)"))
                .and_then(|modified| if modified == 1 {self.id = Some(id); Some(self)} else {None}
                    .log("Row not found (Alias::save)"))
        } else {
            conn.prepare_cached(INSERT_ALIAS).log("Preparing INSERT aliases query (Alias::save)")
                .and_then(|stmt| stmt.query(&[&(self.book_id as i32), &self.name])
                    .log("Executing INSERT aliases query (Alias::save)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| {self.id = Some(row.get::<usize, i32>(0) as usize); self})
                        .log("Inserted id not found (Alias::save)")))
        }
    }

    fn delete(id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(DELETE_ALIAS).log("Preparing DELETE aliases query (Alias::delete)")
            .and_then(|stmt| stmt.execute(&[&(id as i32)])
                .log("Executing DELETE aliases query (Alias::delete)"))
            .and_then(|modified| if modified == 1 {Some(())} else {None}
                .log("Row not found (Alias::delete)"))
    }
}
