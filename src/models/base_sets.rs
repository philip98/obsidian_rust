use chrono::{DateTime, UTC};
use iron::Request;
use postgres::Connection;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder, json};
use std::collections::HashSet;

use handlers::Optionable;
use middleware::RequestBody;
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

impl BaseSet {
    pub fn parse_many(req: &Request) -> Option<Vec<BaseSet>> {
        req.extensions.get::<RequestBody>().log("RequestBody extension could not be found (BaseSet::parse_many)")
            .and_then(|body| json::decode::<Vec<BaseSet>>(&body).log("Parsing vector of BaseSets (BaseSet::parse_many)"))
    }

}

impl Model for BaseSet {
    fn find_id(_: usize, _: usize, _: &Connection, _: &Includes) -> Option<Self> {
        unreachable!()
    }

    fn find_all(_: usize, _: &Connection, _: &Includes) -> Vec<Self> {
        unreachable!()
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Option<Self> {
        if let Some(_) = id {
            unreachable!()
        } else {
            Book::find_id(self.book_id, school_id, conn, &HashSet::new())
                .and_then(|_| Student::find_id(self.student_id, school_id, conn, &HashSet::new()))
                .and_then(|_| conn.prepare_cached(INSERT_BASE_SET)
                    .log("Preparing INSERT base_sets query (BaseSet::save)"))
                .and_then(|stmt| stmt.query(&[&(self.student_id as i32), &(self.book_id as i32), &self.created_at])
                    .log("Executing INSERT base_sets query (BaseSet::save)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| {self.id = Some(row.get::<usize, i32>(0) as usize); self})
                        .log("No id returned (BaseSet::save)")))
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(DELETE_BASE_SET)
            .log("Preparing DELETE base_sets query (BaseSet::delete)")
            .and_then(|stmt| stmt.execute(&[&(id as i32), &(school_id as i32)])
                .log("Executing DELETE base_sets query (BaseSet::delete)"))
            .and_then(|modified| if modified == 1 {Some(())} else {None}
                .log("BaseSet does not exist (BaseSet::delete)"))
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
