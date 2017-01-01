use rustc_serialize::{Decodable, Decoder, json};
use postgres::Connection;
use iron::Request;
use chrono::{DateTime, UTC};

use models::{Includable, Includes, Model};
use models::books::Book;
use handlers::Optionable;
use middleware::RequestBody;

type LentBook = (String, Book);

const QUERY_STUDENT: &'static str = "SELECT name, class_letter, graduation_year FROM students WHERE id = $1";
const QUERY_STUDENTS: &'static str = "SELECT id, name, class_letter, graduation_year FROM students";
const QUERY_LENDINGS: &'static str = "SELECT title, form, isbn, lendings.created_at, books.id FROM lendings, books
    WHERE lendings.person_id=$1 AND lendings.person_type='student' AND lendings.book_id = books.id";
const QUERY_BASE_SETS: &'static str = "SELECT title, form, isbn, base_sets.created_at, books.id FROM base_sets, books
    WHERE base_sets.student_id=$1 AND base_sets.book_id = books.id";

fn find_base_sets(student_id: usize, conn: &Connection) -> Option<Vec<LentBook>> {
    conn.prepare_cached(QUERY_BASE_SETS).log("Preparing SELECT base_sets query (find_base_sets)")
        .and_then(|stmt| stmt.query(&[&(student_id as i32)]).log("Executing SELECT base_sets query (find_base_sets)")
            .map(|rows| rows
                .iter()
                .map(|row| (
                    row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                    Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                        row.get::<usize, String>(0), row.get::<usize, String>(1))
                )).collect()))
}

fn find_lendings(student_id: usize, conn: &Connection) -> Option<Vec<LentBook>> {
    conn.prepare_cached(QUERY_LENDINGS).log("Preparing SELECT lendings query (find_lendings)")
        .and_then(|stmt| stmt.query(&[&(student_id as i32)]).log("Executing SELECT lendings query (find_lendings)")
            .map(|rows| rows
                .iter()
                .map(|row| (
                    row.get::<usize, DateTime<UTC>>(3).to_rfc3339(),
                    Book::new(Some(row.get::<usize, i32>(4) as usize), row.get::<usize, String>(2),
                        row.get::<usize, String>(0), row.get::<usize, String>(1))
                    )
                )
                .collect()))
}

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
    pub fn parse_many(req: &Request) -> Option<Vec<Student>> {
        req.extensions.get::<RequestBody>().log("RequestBody extension could not be found (Student::parse_many)")
            .and_then(|body| json::decode::<Vec<Student>>(&body).log("Parsing vector of Students (Student::parse_many)"))
    }
}

impl Model for Student {
    fn find_id(id: usize, conn: &Connection, includes: &Includes) -> Option<Student> {
        if includes.contains(&Includable::Aliases) {
            None
        } else {
            conn.prepare_cached(QUERY_STUDENT).log("Preparing SELECT students query (Student::find_id)")
                .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT query (Student::find_id)")
                    .map(|rows| {
                        let base_sets = if includes.contains(&Includable::BaseSetBooks) {
                            find_base_sets(id, conn)
                        } else {
                            None
                        };
                        let lendings = if includes.contains(&Includable::LentBooks) {
                            find_lendings(id, conn)
                        } else {
                            None
                        };

                        Student{
                            id: Some(id),
                            name: rows.get(0).get(0),
                            class_letter: rows.get(0).get(1),
                            graduation_year: rows.get(0).get(2),
                            lent_books: lendings,
                            base_sets: base_sets
                        }
                    }))
            }
    }

    fn find_all(conn: &Connection, includes: &Includes) -> Vec<Student> {
        if includes.contains(&Includable::Aliases) {
            vec![]
        } else {
            conn.prepare_cached(QUERY_STUDENTS).log("Preparing SELECT query (Student::find_all)")
                .and_then(|stmt| stmt.query(&[]).log("Executing SELECT query (Student::find_all)")
                    .map(|rows| rows
                        .iter()
                        .map(|row| {
                            let id = row.get::<usize, i32>(0) as usize;
                            let base_sets = if includes.contains(&Includable::BaseSetBooks) {
                                find_base_sets(id, conn)
                            } else {
                                None
                            };

                            let lendings = if includes.contains(&Includable::LentBooks) {
                                find_lendings(id, conn)
                            } else {
                                None
                            };

                            Student{
                                id: Some(id),
                                name: row.get(1),
                                class_letter: row.get(2),
                                graduation_year: row.get(3),
                                lent_books: lendings,
                                base_sets: base_sets
                            }
                        })
                        .collect::<Vec<Student>>())).unwrap_or(vec![])
            }
        }

        fn save(mut self, id: Option<usize>, conn: &Connection) -> Option<Self> {
            match id {
                Some(id) => {
                    conn.prepare_cached("UPDATE students SET name=$2, graduation_year=$3,
                        class_letter=$4 WHERE id=$1").log("Preparing UPDATE query (Student::save)")
                        .and_then(|stmt| stmt.execute(&[&(id as i32), &self.name, &self.graduation_year, &self.class_letter])
                            .log("Executing UPDATE query (Student::save)"))
                        .and_then(|modified| (if modified == 1 {self.id = Some(id); Some(self)} else {None})
                            .log("Row does not exist (Student::save)"))
                },
                None => {
                    conn.prepare_cached("INSERT INTO students (name, graduation_year, class_letter)
                        VALUES ($1, $2, $3) RETURNING id").log("Preparing INSERT query (Student::save)")
                        .and_then(|stmt| stmt.query(&[&self.name, &self.graduation_year, &self.class_letter])
                            .log("Executing INSERT query (Student::save)")
                            .and_then(|rows| rows.iter().next()
                                .map(|row| {self.id = Some(row.get::<usize, i32>(0) as usize); self})
                                .log("Finding inserted id (Student::save)")))
                }
            }
    }

    fn delete(id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached("DELETE FROM students WHERE id=$1").log("Preparing DELETE query (Student::delete)")
            .and_then(|stmt| stmt.execute(&[&(id as i32)]).log("Executing DELETE query (Student::delete)"))
            .and_then(|modified| (if modified == 1 {Some(())} else {None}).log("Row does not exist (Student::delete)"))
    }
}

impl Decodable for Student {
    fn decode<D: Decoder>(d: &mut D) -> Result<Student, D::Error> {
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