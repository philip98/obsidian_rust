use chrono::{DateTime, UTC};
use postgres::Connection;
use postgres::rows::Row;
use rustc_serialize::{Decodable, Decoder};

use error::ObsidianError;
use models::{Model, Includes, Includable};
use models::books::Book;

#[derive(RustcEncodable, Debug)]
struct LentBook{
    id: usize,
    created_at: String,
    book: Book
}

const QUERY_TEACHER: &'static str = "SELECT id, name FROM teachers WHERE id=$1 AND school_id=$2";
const QUERY_TEACHERS: &'static str = "SELECT id, name FROM teachers WHERE school_id=$1";
const QUERY_LENDINGS: &'static str = "SELECT title, form, isbn, lendings.created_at, books.id, lendings.id FROM lendings, books
WHERE lendings.person_type='teacher' AND lendings.person_id=$1 AND lendings.book_id = books.id";

const INSERT_TEACHER: &'static str = "INSERT INTO teachers (name, school_id) VALUES ($1, $2) RETURNING id";
const UPDATE_TEACHER: &'static str = "UPDATE teachers SET name=$2 WHERE id=$1 AND school_id=$3";
const DELETE_TEACHER: &'static str = "DELETE FROM teachers WHERE id=$1 AND school_id=$2";

#[derive(RustcEncodable, Debug)]
pub struct Teacher {
    id: Option<usize>,
    name: String,
    lent_books: Option<Vec<LentBook>>
}

impl Teacher {
    fn from_db(conn: &Connection, includes: &Includes, row: Row) -> Result<Teacher, ObsidianError> {
        let id = row.get::<usize, i32>(0) as usize;
        let lent_books = if includes.contains(&Includable::LentBooks) {
            let stmt = try!(conn.prepare_cached(QUERY_LENDINGS));
            let rows = try!(stmt.query(&[&(id as i32)]));
            Some(rows.iter()
                .map(|row| LentBook {
                    id: row.get::<usize, i32>(5) as usize,
                    created_at: row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                    book: Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                        row.get::<usize, String>(0), row.get::<usize, String>(1))
                })
                .collect::<Vec<LentBook>>())
        } else {
            None
        };
        Ok(Teacher {
            id: Some(id),
            name: row.get::<usize, String>(1),
            lent_books: lent_books
        })
    }
}

impl Model for Teacher {
    fn find_id(id: usize, school_id: usize, conn: &Connection, includes: &Includes) -> Result<Self, ObsidianError> {
        does_not_support!(BaseSetBooks, includes);
        does_not_support!(Aliases, includes);
        let stmt = try!(conn.prepare_cached(QUERY_TEACHER));
        let rows = try!(stmt.query(&[&(id as i32), &(school_id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("Teacher")));
        let teacher = try!(Teacher::from_db(conn, includes, row));
        if teacher.id == Some(id) {
            Ok(teacher)
        } else {
            Err(ObsidianError::RecordNotFound("Teacher"))
        }
    }

    fn find_all(school_id: usize, conn: &Connection, includes: &Includes) -> Result<Vec<Self>, ObsidianError> {
        does_not_support!(BaseSetBooks, includes);
        does_not_support!(Aliases, includes);
        let stmt = try!(conn.prepare_cached(QUERY_TEACHERS));
        let rows = try!(stmt.query(&[&(school_id as i32)]));
        rows.iter()
            .map(|row| Teacher::from_db(conn, includes, row))
            .collect::<Result<Vec<Teacher>, ObsidianError>>()
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError> {
        if let Some(id) = id {
            let stmt = try!(conn.prepare_cached(UPDATE_TEACHER));
            let modified = try!(stmt.execute(&[&(id as i32), &self.name, &(school_id as i32)]));
            if modified == 1  {
                self.id = Some(id);
                Ok(self)
            } else {
                Err(ObsidianError::RecordNotFound("Teacher"))
            }
        } else {
            let stmt = try!(conn.prepare_cached(INSERT_TEACHER));
            let rows = try!(stmt.query(&[&self.name, &(school_id as i32)]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_TEACHER));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("Teacher"))
        }
    }
}

impl Decodable for Teacher {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Teacher", 1, |d| d
            .read_struct_field("name", 0, D::read_str)
                .map(|name| Teacher{id: None, name: name, lent_books: None}))
        .or_else(|_| d.read_struct("Teacher", 2, |d| {
            let id = try!(d.read_struct_field("id", 0, Option::<usize>::decode));
            let name = try!(d.read_struct_field("name", 1, D::read_str));
            Ok(Teacher{id: id, name: name, lent_books: None})
        }))
    }
}
