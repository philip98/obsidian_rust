use chrono::{DateTime, UTC};
use postgres::Connection;
use postgres::rows::Row;
use rustc_serialize::{Decodable, Decoder};

use handlers::Optionable;
use models::{Model, Includes, Includable};
use models::books::Book;

type LentBook = (String, Book);

const QUERY_TEACHER: &'static str = "SELECT id, name FROM teachers WHERE id=$1 AND school_id=$2";
const QUERY_TEACHERS: &'static str = "SELECT id, name FROM teachers WHERE school_id=$1";
const QUERY_LENDINGS: &'static str = "SELECT title, form, isbn, lendings.created_at, books.id FROM lendings, books
WHERE lendings.person_type='teacher' AND lendings.person_id=$1 AND lendings.book_id = books.id";

const INSERT_TEACHER: &'static str = "INSERT INTO teachers (name, school_id) VALUES ($1, $2) RETURNING id";
const UPDATE_TEACHER: &'static str = "UPDATE teachers SET name=$2 WHERE id=$1 AND school_id=$3";
const DELETE_TEACHER: &'static str = "DELETE FROM teachers WHERE id=$1 AND school_id=$2";

#[derive(RustcEncodable)]
pub struct Teacher {
    id: Option<usize>,
    name: String,
    lent_books: Option<Vec<LentBook>>
}

impl Teacher {
    fn from_db(conn: &Connection, includes: &Includes, row: Row) -> Teacher {
        let id = row.get::<usize, i32>(0) as usize;
        let lent_books = if includes.contains(&Includable::LentBooks) {
            conn.prepare_cached(QUERY_LENDINGS).log("Preparing SELECT lendings query (Teacher::from_db)")
                .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT lendings query (Teacher::from_db)")
                    .map(|rows| rows
                        .iter()
                        .map(|row| (
                            row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                            Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                                row.get::<usize, String>(0), row.get::<usize, String>(1))
                        ))
                        .collect::<Vec<LentBook>>()))
        } else {
            None
        };
        Teacher {
            id: Some(id),
            name: row.get::<usize, String>(1),
            lent_books: lent_books
        }
    }
}

impl Model for Teacher {
    fn find_id(id: usize, school_id: usize, conn: &Connection, includes: &Includes) -> Option<Self> {
        if includes.contains(&Includable::BaseSetBooks) || includes.contains(&Includable::Aliases) {
            None.log(&format!("Include params {:?} not supported", includes))
        } else {
            conn.prepare_cached(QUERY_TEACHER).log("Preparing SELECT teachers query (Teacher::find_id)")
                .and_then(|stmt| stmt.query(&[&(id as i32), &(school_id as i32)])
                    .log("Executing SELECT teachers query (Teacher::find_id)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| Teacher::from_db(conn, includes, row))
                        .log("No teachers found (Teacher::find_id)")))
                .and_then(|teacher| if teacher.id == Some(id) {Some(teacher)} else {None}
                    .log("IDs don't match (Teacher::find_id)"))
        }
    }

    fn find_all(school_id: usize, conn: &Connection, includes: &Includes) -> Vec<Self> {
        if includes.contains(&Includable::BaseSetBooks) || includes.contains(&Includable::Aliases) {
            None::<Teacher>.log(&format!("Include params {:?} not supported", includes));
            vec![]
        } else {
            conn.prepare_cached(QUERY_TEACHERS).log("Preparing SELECT teachers query (Teacher::find_all)")
                .and_then(|stmt| stmt.query(&[&(school_id as i32)])
                    .log("Executing SELECT teachers query (Teacher::find_all)")
                    .map(|rows| rows
                        .iter()
                        .map(|row| Teacher::from_db(conn, includes, row))
                        .collect::<Vec<Teacher>>()))
                .unwrap_or(vec![])
        }
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Option<Self> {
        if let Some(id) = id {
            conn.prepare_cached(UPDATE_TEACHER).log("Preparing UPDATE teachers query (Teacher::save)")
                .and_then(|stmt| stmt.execute(&[&(id as i32), &self.name, &(school_id as i32)])
                    .log("Executing UPDATE teachers query (Teacher::save)"))
                .and_then(|modified| if modified == 1 {self.id = Some(id); Some(self)} else {None}
                    .log("No teachers found (Teacher::save)"))
        } else {
            conn.prepare_cached(INSERT_TEACHER).log("Preparing INSERT teachers query (Teacher::save)")
                .and_then(|stmt| stmt.query(&[&self.name, &(school_id as i32)])
                    .log("Executing INSERT teachers query (Teacher::save)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| {self.id = Some(row.get::<usize, i32>(0) as usize); self})
                        .log("No id retuned (Teachers::save)")))
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(DELETE_TEACHER).log("Preparing DELETE teachers query (Teacher::delete)")
            .and_then(|stmt| stmt.execute(&[&(id as i32), &(school_id as i32)])
                .log("Executing DELETE teachers query (Teacher::delete)"))
            .and_then(|modified| if modified == 1 {Some(())} else {None}.log("No teachers found (Teacher::delete)"))
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
