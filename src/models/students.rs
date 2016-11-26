use rustc_serialize::json;
use postgres::Connection;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Student {
    name: String,
    class_letter: String,
    graduation_year: i32
}

impl Student {
    pub fn find_id(id: usize, conn: &Connection) -> Option<Student> {
        conn.prepare_cached("SELECT name, class_letter, graduation_year FROM students
            WHERE id = $1")
            .and_then(|stmt| stmt.query(&[&(id as i32)])
                .map(|rows| Student{
                    name: rows.get(0).get(0),
                    class_letter: rows.get(0).get(1),
                    graduation_year: rows.get(0).get(2)
                }))
            .ok()
    }

    pub fn find_all(conn: &Connection) -> Vec<Student> {
        conn.prepare_cached("SELECT name, class_letter, graduation_year FROM students")
            .and_then(|stmt| stmt.query(&[])
                .map(|rows| rows.iter().map(|row|
                    Student{
                        name: row.get(0),
                        class_letter: row.get(1),
                        graduation_year: row.get(2)
                    }).collect::<Vec<Student>>()
                )
            ).unwrap_or(vec![])
    }

    pub fn read(body: &str) -> Option<Student> {
        if let Ok(res) = json::decode::<Student>(body) {
            Some(res)
        } else {
            None
        }
    }

    pub fn read_many(body: &str) -> Vec<Student> {
        if let Ok(res) = json::decode::<Vec<Student>>(body) {
            res
        } else {
            vec![]
        }
    }
}

#[test]
fn serialisation_works() {
    let s = Student{name: "Philip Schlösser".to_string(), class_letter: String::new(),
        graduation_year: 2016};
    json::encode(&s).unwrap();
}

#[test]
fn reading_works() {
    Student::read("{\"name\": \"דויד לבי\",\"class_letter\": \"c\",\"graduation_year\":2015}").unwrap();
    assert_eq!(Student::read_many("[{\"name\": \"PS\", \"class_letter\": \"a\", \"graduation_year\":2011},
    {\"name\": \"JV\", \"class_letter\": \"\", \"graduation_year\": 2017}]").len(), 2);
}
