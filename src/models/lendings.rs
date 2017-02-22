use chrono::{DateTime, UTC};
use postgres::Connection;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use std::collections::HashSet;

use error::ObsidianError;
use models::{Includes, Model};
use models::books::Book;
use models::students::Student;
use models::teachers::Teacher;

const INSERT_ST_LENDING: &'static str = "INSERT INTO lendings (person_type, person_id, book_id, created_at)
VALUES ('student', $1, $2, $3) RETURNING id";
const INSERT_TE_LENDING: &'static str = "INSERT INTO lendings (person_type, person_id, book_id, created_at)
VALUES ('teacher', $1, $2, $3) RETURNING id";
const DELETE_LENDING: &'static str = "DELETE FROM lendings WHERE lendings.id=$1 AND EXISTS
(SELECT * FROM books WHERE book.id = lendings.book_id AND book.school_id=$2)";

#[derive(Debug)]
enum Person {
    Student(usize),
    Teacher(usize)
}

#[derive(Debug)]
pub struct Lending {
    id: Option<usize>,
    created_at: DateTime<UTC>,
    person: Person,
    book_id: usize
}

impl Model for Lending {
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
            let (id, query) = match self.person {
                Person::Student(id) => {
                    try!(Student::find_id(id, school_id, conn, &HashSet::new()));
                    (id, INSERT_ST_LENDING)
                },
                Person::Teacher(id) => {
                    try!(Teacher::find_id(id, school_id, conn, &HashSet::new()));
                    (id, INSERT_TE_LENDING)
                }
            };
            try!(Book::find_id(self.book_id, school_id, conn, &HashSet::new()));
            let stmt = try!(conn.prepare_cached(query));
            let rows = try!(stmt.query(&[&(id as i32), &(self.book_id as i32), &self.created_at]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_LENDING));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("Lending"))
        }
    }
}

impl Encodable for Lending {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Lending", 5, |s| {
            try!(s.emit_struct_field("id", 0, |s| self.id.encode(s)));
            try!(s.emit_struct_field("created_at", 1, |s| s.emit_str(&self.created_at.to_rfc3339())));
            match self.person {
                Person::Student(id) => {
                    try!(s.emit_struct_field("person_type", 2, |s| s.emit_str("student")));
                    try!(s.emit_struct_field("person_id", 3, |s| s.emit_usize(id)));
                },
                Person::Teacher(id) => {
                    try!(s.emit_struct_field("person_type", 2, |s| s.emit_str("teacher")));
                    try!(s.emit_struct_field("person_id", 3, |s| s.emit_usize(id)));
                }
            }
            try!(s.emit_struct_field("book_id", 4, |s| s.emit_usize(self.book_id)));
            Ok(())
        })
    }
}

impl Decodable for Lending {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Lending", 3, |d| {
            let person_type = try!(d.read_struct_field("person_type", 0, D::read_str));
            let person = if person_type == "student" {
                Person::Student(try!(d.read_struct_field("person_id", 1, D::read_usize)))
            } else if person_type == "teacher" {
                Person::Teacher(try!(d.read_struct_field("person_id", 1, D::read_usize)))
            } else {
                return Err(d.error("person_type must be either 'student' or 'teacher'"))
            };
            let book_id = try!(d.read_struct_field("book_id", 2, D::read_usize));
            Ok(Lending{
                id: None,
                created_at: UTC::now(),
                person: person,
                book_id: book_id
            })
        }).or_else(|_| d.read_struct("Lending", 4, |d| {
            let id = try!(d.read_struct_field("id", 0, Option::<usize>::decode));
            let person_type = try!(d.read_struct_field("person_type", 1, D::read_str));
            let person = if person_type == "student" {
                Person::Student(try!(d.read_struct_field("person_id", 2, D::read_usize)))
            } else if person_type == "teacher" {
                Person::Teacher(try!(d.read_struct_field("person_id", 2, D::read_usize)))
            } else {
                return Err(d.error("person_type must be either 'student' or 'teacher'"))
            };
            let book_id = try!(d.read_struct_field("book_id", 3, D::read_usize));
            Ok(Lending{
                id: id,
                created_at: UTC::now(),
                person: person,
                book_id: book_id
            })
        }))
    }
}
