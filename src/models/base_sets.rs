use chrono::{DateTime, UTC};
use postgres::Connection;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use std::collections::HashSet;

use error::ObsidianError;
use models::{Model, Includes};
use models::students::Student;
use models::books::Book;

const INSERT_BASE_SET: &'static str = "INSERT INTO base_sets (student_id, book_id, created_at) VALUES ($1, $2, $3) RETURNING id";
const DELETE_BASE_SET: &'static str = "DELETE FROM base_sets WHERE base_sets.id=$1 AND EXISTS
(SELECT * FROM books WHERE books.id = base_sets.book_id AND book.school_id = $2)";

pub struct BaseSet {
    id: Option<usize>,
    student_id: usize,
    book_id: usize,
    created_at: DateTime<UTC>
}

impl Model for BaseSet {
    fn find_id(_: usize, _: usize, _: &Connection, _: &Includes) -> Result<Self, ObsidianError> {
        unreachable!()
    }

    fn find_all(_: usize, _: &Connection, _: &Includes) -> Result<Vec<Self>, ObsidianError> {
        unreachable!()
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError> {
        if let Some(_) = id {
            unreachable!()
        } else {
            try!(Book::find_id(self.book_id, school_id, conn, &HashSet::new()));
            try!(Student::find_id(self.book_id, school_id, conn, &HashSet::new()));
            let stmt = try!(conn.prepare_cached(INSERT_BASE_SET));
            let rows = try!(stmt.query(&[&(self.student_id as i32), &(self.book_id as i32),
                &self.created_at]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_BASE_SET));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("BaseSet"))
        }
    }
}

impl Encodable for BaseSet {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("BaseSet", 4, |s| {
            try!(s.emit_struct_field("id", 0, |s| self.id.encode(s)));
            try!(s.emit_struct_field("student_id", 1, |s| s.emit_usize(self.student_id)));
            try!(s.emit_struct_field("book_id", 2, |s| s.emit_usize(self.book_id)));
            try!(s.emit_struct_field("created_at", 3, |s| s.emit_str(&self.created_at.to_rfc3339())));
            Ok(())
        })
    }
}

impl Decodable for BaseSet {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("BaseSet", 2, |d| {
            let student_id = try!(d.read_struct_field("student_id", 0, D::read_usize));
            let book_id = try!(d.read_struct_field("book_id", 1, D::read_usize));
            Ok(BaseSet{
                id: None,
                student_id: student_id,
                book_id: book_id,
                created_at: UTC::now()
            })
        }).or_else(|_| d.read_struct("BaseSet", 3, |d| {
            let id = try!(d.read_struct_field("id", 0, Option::<usize>::decode));
            let student_id = try!(d.read_struct_field("student_id", 1, D::read_usize));
            let book_id = try!(d.read_struct_field("book_id", 2, D::read_usize));
            Ok(BaseSet{
                id: id,
                student_id: student_id,
                book_id: book_id,
                created_at: UTC::now()
            })
        }))
    }
}
