use rustc_serialize::json;
use postgres::Connection;
use iron::Request;

use super::Model;
use middleware::RequestBody;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Student {
    id: Option<usize>,
    name: String,
    class_letter: String,
    graduation_year: i32
}

impl Student {
    pub fn parse_many(req: &Request) -> Option<Vec<Student>> {
        req.extensions.get::<RequestBody>()
            .and_then(|body| json::decode::<Vec<Student>>(&body)
                .map_err(|err| println!("{}", err)).ok())
    }
}

impl Model for Student {
    fn find_id(id: usize, conn: &Connection) -> Option<Student> {
        conn.prepare_cached("SELECT name, class_letter, graduation_year FROM students
            WHERE id = $1")
            .and_then(|stmt| stmt.query(&[&(id as i32)])
                .map(|rows| Student{
                    id: Some(id),
                    name: rows.get(0).get(0),
                    class_letter: rows.get(0).get(1),
                    graduation_year: rows.get(0).get(2)
                }))
            .ok()
    }

    fn find_all(conn: &Connection) -> Vec<Student> {
        conn.prepare_cached("SELECT id, name, class_letter, graduation_year FROM students")
            .and_then(|stmt| stmt.query(&[])
                .map(|rows| rows.iter().map(|row|
                    Student{
                        id: Some(row.get::<usize, i32>(0) as usize),
                        name: row.get(1),
                        class_letter: row.get(2),
                        graduation_year: row.get(3)
                    }).collect::<Vec<Student>>()
                )
            ).unwrap_or(vec![])
    }

    fn save(mut self, id: Option<usize>, conn: &Connection) -> Option<Self> {
        match id {
            Some(id) => {
                conn.prepare_cached("UPDATE students SET name=$2, graduation_year=$3,
                    class_letter=$4 WHERE id=$1")
                    .and_then(|stmt| stmt.execute(&[&(id as i32), &self.name, &self.graduation_year, &self.class_letter]))
                    .ok()
                    .and_then(|modified| if modified == 1 {self.id = Some(id); Some(self)} else {None})
            },
            None => {
                conn.prepare_cached("INSERT INTO students (name, graduation_year, class_letter)
                    VALUES ($1, $2, $3) RETURNING id")
                    .map_err(|err| {println!("Error while saving student: {}", err);})
                    .ok()
                    .and_then(|stmt| stmt.query(&[&self.name, &self.graduation_year, &self.class_letter])
                    .map_err(|err| {println!("Error while saving student: {}", err);})
                        .ok()
                        .and_then(|rows| rows.iter().next()
                            .map(|row| {self.id = Some(row.get::<usize, i32>(0) as usize); self})))
            }
        }
    }

    fn delete(id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached("DELETE FROM students WHERE id=$1")
            .and_then(|stmt| stmt.execute(&[&(id as i32)]))
            .ok()
            .and_then(|modified| if modified == 1 {Some(())} else {None})
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
