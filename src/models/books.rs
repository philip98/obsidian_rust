use models::Model;

use postgres::Connection;
use postgres::rows::Row;

use handlers::Optionable;
use models::Includes;

const QUERY_BOOK: &'static str = "SELECT id, isbn, title, form FROM books WHERE id = $1";
const QUERY_BOOKS: &'static str = "SELECT id, isbn, title, form FROM books";

const INSERT_BOOK: &'static str = "INSERT INTO books (isbn, title, form) VALUES ($1, $2, $3) RETURNING id";
const UPDATE_BOOK: &'static str = "UPDATE books SET isbn=$2, title=$3, form=$4 WHERE id=$1";
const DELETE_BOOK: &'static str = "DELETE FROM books WHERE id=$1";

#[derive(RustcEncodable, RustcDecodable)]
pub struct Book {
    id: Option<usize>,
    isbn: String,
    title: String,
    form: String
}

impl Book {
    pub fn new(id: Option<usize>, isbn: String, title: String, form: String) -> Book {
        Book {
            id: id,
            isbn: isbn,
            title: title,
            form: form
        }
    }

    fn from_db(_: &Connection, _: &Includes, row: Row) -> Book {
        Book {
            id: Some(row.get::<usize, i32>(0) as usize),
            isbn: row.get::<usize, String>(1),
            title: row.get::<usize, String>(2),
            form: row.get::<usize, String>(3)
        }
    }
}

impl Model for Book {
    fn find_id(id: usize, conn: &Connection, includes: &Includes) -> Option<Self> {
        conn.prepare_cached(QUERY_BOOK).log("Preparing SELECT books query (Book::find_id)")
            .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT books query (Book::find_id)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .map(|row| Book::from_db(conn, includes, row))
                    .log("No books found (Book::find_id)")))
            .and_then(|book| (if book.id == Some(id) {Some(book)} else {None})
                .log("Book has wrong id (Book::find_id)"))
    }

    fn find_all(conn: &Connection, includes: &Includes) -> Vec<Self> {
        conn.prepare_cached(QUERY_BOOKS).log("Preparing SELECT books query (Book::find_all)")
            .and_then(|stmt| stmt.query(&[]).log("Executing SELECT books query (Book::find_all)")
                .map(|rows| rows
                    .iter()
                    .map(|row| Book::from_db(conn, includes, row))
                    .collect::<Vec<Book>>()))
            .unwrap_or(vec![])
    }

    fn save(mut self, id: Option<usize>, conn: &Connection) -> Option<Self> {
        if let Some(id) = id {
            conn.prepare_cached(UPDATE_BOOK).log("Preparing UPDATE books query (Book::save)")
                .and_then(|stmt| stmt.execute(&[&(id as i32), &self.isbn, &self.title, &self.form])
                    .log("Executing UPDATE books query (Book::save)"))
                .and_then(|modified| (if modified == 1 {self.id = Some(id);Some(self)} else {None})
                    .log("Row does not exist"))
        } else {
            conn.prepare_cached(INSERT_BOOK).log("Preparing INSERT books query (Book::save)")
                .and_then(|stmt| stmt.query(&[&self.isbn, &self.title, &self.form])
                    .log("Executing INSERT books query (Books::save)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| {
                            self.id = Some(row.get::<usize, i32>(0) as usize);
                            self
                        })
                        .log("Finding inserted id")))
        }
    }

    fn delete(id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(DELETE_BOOK).log("Preparing DELETE books query (Book::delete)")
            .and_then(|stmt| stmt.execute(&[&(id as i32)])
                .log("Executing DELETE books query (Book::delete)"))
            .and_then(|modified| (if modified == 1 {Some(())} else {None})
                .log("Row does not exist"))
    }
}
