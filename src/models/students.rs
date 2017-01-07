use rustc_serialize::{Decodable, Decoder};
use postgres::Connection;
use postgres::rows::Row;
use chrono::{DateTime, UTC};

use error::ObsidianError;
use models::{Includable, Includes, Model};
use models::books::Book;

#[derive(RustcEncodable)]
struct LentBook {
    id: usize,
    created_at: String,
    book: Book,
}

const QUERY_STUDENT: &'static str = "SELECT id, name, class_letter, graduation_year FROM students
WHERE id = $1, school_id=$2";
const QUERY_STUDENTS: &'static str = "SELECT id, name, class_letter, graduation_year FROM students
WHERE school_id=$1";
const QUERY_LENDINGS: &'static str = "SELECT title, form, isbn, lendings.created_at, books.id, lendings.id FROM lendings, books
    WHERE lendings.person_id=$1 AND lendings.person_type='student' AND lendings.book_id = books.id";
const QUERY_BASE_SETS: &'static str = "SELECT title, form, isbn, base_sets.created_at, books.id, base_sets.id FROM base_sets, books
    WHERE base_sets.student_id=$1 AND base_sets.book_id = books.id";

const INSERT_STUDENT: &'static str = "INSERT INTO students (name, graduation_year, class_letter, school_id)
    VALUES ($1, $2, $3, $4) RETURNING id";
const UPDATE_STUDENT: &'static str = "UPDATE students SET name=$2, graduation_year=$3,
    class_letter=$4 WHERE id=$1 AND school_id=$5";
const DELETE_STUDENT: &'static str = "DELETE FROM students WHERE id=$1 AND school_id=$2";

#[derive(RustcEncodable)]
pub struct Student {
    id: Option<usize>,
    name: String,
    class_letter: String,
    graduation_year: i32,
    lent_books: Option<Vec<LentBook>>,
    base_sets: Option<Vec<LentBook>>
}

impl Student {
    fn find_base_sets(student_id: usize, conn: &Connection) -> Result<Vec<LentBook>, ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_BASE_SETS));
        let rows = try!(stmt.query(&[&(student_id as i32)]));
        Ok(rows.iter()
            .map(|row| LentBook {
                id: row.get::<usize, i32>(5) as usize,
                created_at: row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                book: Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                    row.get::<usize, String>(0), row.get::<usize, String>(1))
            })
            .collect::<Vec<LentBook>>())
    }

    fn find_lendings(student_id: usize, conn: &Connection) -> Result<Vec<LentBook>, ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_LENDINGS));
        let rows = try!(stmt.query(&[&(student_id as i32)]));
        Ok(rows.iter()
            .map(|row| LentBook {
                id: row.get::<usize, i32>(5) as usize,
                created_at: row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                book: Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                    row.get::<usize, String>(0), row.get::<usize, String>(1))
            })
            .collect::<Vec<LentBook>>())
    }

    fn from_db(conn: &Connection, includes: &Includes, row: Row) -> Result<Student, ObsidianError> {
        let id = row.get::<usize, i32>(0) as usize;
        let base_sets = if includes.contains(&Includable::BaseSetBooks) {
            Some(try!(Student::find_base_sets(id, conn)))
        } else {
            None
        };
        let lendings = if includes.contains(&Includable::LentBooks) {
            Some(try!(Student::find_lendings(id, conn)))
        } else {
            None
        };

        Ok(Student{
            id: Some(id),
            name: row.get(1),
            class_letter: row.get(2),
            graduation_year: row.get(3),
            lent_books: lendings,
            base_sets: base_sets
        })
    }
}

impl Model for Student {
    fn find_id(id: usize, school_id: usize, conn: &Connection, includes: &Includes) -> Result<Student, ObsidianError> {
        does_not_support!(Aliases, includes);
        let stmt = try!(conn.prepare_cached(QUERY_STUDENT));
        let rows = try!(stmt.query(&[&(id as i32), &(school_id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("Student")));
        let student = try!(Student::from_db(conn, includes, row));
        if student.id == Some(id) {
            Ok(student)
        } else {
            Err(ObsidianError::RecordNotFound("Student"))
        }
    }

    fn find_all(school_id: usize, conn: &Connection, includes: &Includes) -> Result<Vec<Student>, ObsidianError> {
        does_not_support!(Aliases, includes);
        let stmt = try!(conn.prepare_cached(QUERY_STUDENTS));
        let rows = try!(stmt.query(&[&(school_id as i32)]));
        rows.iter()
            .map(|row| Student::from_db(conn, includes, row))
            .collect::<Result<Vec<Student>, ObsidianError>>()
    }

    fn save(mut self, id: Option<usize>, school_id: usize, conn: &Connection) -> Result<Self, ObsidianError> {
        if let Some(id) = id {
            let stmt = try!(conn.prepare_cached(UPDATE_STUDENT));
            let modified = try!(stmt.execute(&[&(id as i32), &self.name, &self.graduation_year,
                &self.class_letter, &(school_id as i32)]));
            if modified == 1 {
                self.id = Some(id);
                Ok(self)
            } else {
                Err(ObsidianError::RecordNotFound("Student"))
            }
        } else {
            let stmt = try!(conn.prepare_cached(INSERT_STUDENT));
            let rows = try!(stmt.query(&[&self.name, &self.graduation_year, &self.class_letter,
                &(school_id as i32)]));
            let row = rows.iter().next().unwrap();
            self.id = Some(row.get::<usize, i32>(0) as usize);
            Ok(self)
        }
    }

    fn delete(id: usize, school_id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_STUDENT));
        let modified = try!(stmt.execute(&[&(id as i32), &(school_id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("Student"))
        }
    }
}

impl Decodable for Student {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Student", 3, |d| {
            let name = try!(d.read_struct_field("name", 0, |d|
                d.read_str()));
            let class_letter = try!(d.read_struct_field("class_letter", 1, |d|
                d.read_str()));
            let graduation_year = try!(d.read_struct_field("graduation_year", 2, |d|
                d.read_i32()));
            Ok(Student {
                id: None,
                name: name,
                class_letter: class_letter,
                graduation_year: graduation_year,
                lent_books: None,
                base_sets: None
            })
        }).or_else(|_|
            d.read_struct("Student", 4, |d| {
                let id = try!(d.read_struct_field("id", 0, |d| Option::<usize>::decode(d)));
                let name = try!(d.read_struct_field("name", 1, |d|
                    d.read_str()));
                let class_letter = try!(d.read_struct_field("class_letter", 2, |d|
                    d.read_str()));
                let graduation_year = try!(d.read_struct_field("graduation_year", 3, |d|
                    d.read_i32()));
                Ok(Student {
                    id: id,
                    name: name,
                    class_letter: class_letter,
                    graduation_year: graduation_year,
                    lent_books: None,
                    base_sets: None
                })
            }))
    }
}

#[test]
fn serialisation_works() {
    let s = Student{id: None, name: "Philip Schlösser".to_string(), class_letter: String::new(),
        graduation_year: 2016};
    println!("{}", json::encode(&s).unwrap());
    let v = vec![s, Student{id: Some(5), name: "aoidhfaio".to_string(), class_letter: "abc".to_string(),
        graduation_year: 2017}];
    println!("{}", json::encode(&v).unwrap());
}

#[test]
fn reading_works() {
    Student::from_str("{\"name\": \"דויד לבי\",\"class_letter\": \"c\",\"graduation_year\":2015}").unwrap();
    assert_eq!(Student::many_from_str("[{\"name\": \"PS\", \"class_letter\": \"a\", \"graduation_year\":2011},
    {\"name\": \"JV\", \"class_letter\": \"\", \"graduation_year\": 2017}]").len(), 2);
}
