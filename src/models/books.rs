use models::Model;

use postgres::Connection;
use postgres::rows::Row;
use rustc_serialize::{Decoder, Decodable};

use error::ObsidianError;
use models::{Includes, Includable};
use models::aliases::Alias;

const QUERY_BOOK: &'static str = "SELECT id, isbn, title, form FROM books WHERE id = $1 AND school_id = $2";
const QUERY_BOOKS: &'static str = "SELECT id, isbn, title, form FROM books WHERE school_id = $1";
const QUERY_ALIASES: &'static str = "SELECT id, name FROM aliases WHERE book_id=$1";

const INSERT_BOOK: &'static str = "INSERT INTO books (isbn, title, form, school_id) VALUES ($1, $2, $3, $4) RETURNING id";
const UPDATE_BOOK: &'static str = "UPDATE books SET isbn=$2, title=$3, form=$4 WHERE id=$1 AND school_id=$5";
const DELETE_BOOK: &'static str = "DELETE FROM books WHERE id=$1 AND school_id=$2";

#[derive(RustcEncodable, Debug)]
pub struct Book {
    id: Option<usize>,
    isbn: String,
    title: String,
    form: String,
    aliases: Option<Vec<Alias>>
}

impl Book {
    pub fn new(id: Option<usize>, isbn: String, title: String, form: String) -> Book {
        Book {
            id: id,
            isbn: isbn,
            title: title,
            form: form,
            aliases: None
        }
    }

    fn from_db(conn: &Connection, includes: &Includes, row: Row) -> Result<Book, ObsidianError> {
        let id = row.get::<usize, i32>(0) as usize;
        let aliases = if includes.contains(&Includable::Aliases) {
            let stmt = try!(conn.prepare_cached(QUERY_ALIASES));
            let rows = try!(stmt.query(&[&(id as i32)]));
            let aliases = rows
                .iter()
                .map(|row| Alias::new(Some(row.get::<usize, i32>(0) as usize),
                    id, row.get::<usize, String>(1)))
                .collect::<Vec<Alias>>();
            Some(aliases)
        } else {
            None
        };
        Ok(Book {
            id: Some(id),
            isbn: row.get::<usize, String>(1),
            title: row.get::<usize, String>(2),
            form: row.get::<usize, String>(3),
            aliases: aliases
        })
    }
}

impl Model for Book {
    fn find_id(id: usize, school_id: usize, conn: &Connection, includes: &Includes) -> Result<Self, ObsidianError> {
        does_not_support!(BaseSetBooks, includes);
        does_not_support!(LentBooks, includes);
        let stmt = try!(conn.prepare_cached(QUERY_BOOK));
        let rows = try!(stmt.query(&[&(id as i32), &(school_id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("Book")));
        let book = try!(Book::from_db(conn, includes, row));
        if book.id == Some(id) {
            Ok(book)
        } else {
            Err(ObsidianError::RecordNotFound("Book"))
        }
    }

    fn find_all(school_id: usize, conn: &Connection, includes: &Includes) -> Result<Vec<Self>, ObsidianError> {
        does_not_support!(BaseSetBooks, includes);
        does_not_support!(LentBooks, includes);
        let stmt = try!(conn.prepare_cached(QUERY_BOOKS));
        let rows = try!(stmt.query(&[&(school_id as i32)]));
        rows
            .iter()
            .map(|row| Book::from_db(conn, includes, row))
            .collect::<Result<Vec<Book>, ObsidianError>>()
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError> {
        if let Some(id) = id {
            let stmt = try!(conn.prepare_cached(UPDATE_BOOK));
            let modified = try!(stmt.execute(&[&(id as i32), &self.isbn, &self.title,
                &self.form, &(school_id as i32)]));
            if modified == 1 {
                self.id = Some(id);
                Ok(self)
            } else {
                Err(ObsidianError::RecordNotFound("Book"))
            }
        } else {
            let stmt = try!(conn.prepare_cached(INSERT_BOOK));
            let rows = try!(stmt.query(&[&self.isbn, &self.title, &self.form,
                &(school_id as i32)]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_BOOK));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("Book"))
        }
    }
}

impl Decodable for Book {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Book", 3, |d| {
            let isbn = try!(d.read_struct_field("isbn", 0, D::read_str));
            let title = try!(d.read_struct_field("title", 1, D::read_str));
            let form = try!(d.read_struct_field("form", 2, D::read_str));
            Ok(Book {
                id: None,
                isbn: isbn,
                title: title,
                form: form,
                aliases: None
            })
        })
        .or_else(|_| d.read_struct("Book", 4, |d| {
            let id = try!(d.read_struct_field("id", 0, Option::<usize>::decode));
            let isbn = try!(d.read_struct_field("isbn", 1, D::read_str));
            let title = try!(d.read_struct_field("title", 2, D::read_str));
            let form = try!(d.read_struct_field("form", 3, D::read_str));
            Ok(Book {
                id: id,
                isbn: isbn,
                title: title,
                form: form,
                aliases: None
            })
        }))
    }
}
