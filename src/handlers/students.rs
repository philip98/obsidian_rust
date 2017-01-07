use chrono::UTC;
use iron::{Request, IronResult, Response};
use postgres::Connection;

use error::ObsidianError;
use handlers::{check_content_type, get_db, get_id, get_includes, get_school_id, parse, serialise};
use models::Model;
use models::students::Student;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let students = try!(Student::find_all(school_id, conn, &includes));
    println!("[{}] Successfully handled students::index request (include={:?})", UTC::now().format("%FT%T%:z"), &includes);
    respond_with!(Ok, students)
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let student = try!(Student::find_id(id, school_id, conn, &includes));
    println!("[{}] Successfully handled students::show request (include={:?})", UTC::now().format("%FT%T%:z"), &includes);
    respond_with!(Ok, student)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let student = try!(parse::<Student>(req));
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let student = try!(student.save(Some(id), school_id, conn));
    println!("[{}] Successfully handled students::edit request", UTC::now().format("%FT%T%:z"));
    respond_with!(Ok, student)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    fn single(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let student = try!(parse::<Student>(req));
        let student = try!(student.save(None, school_id, conn));
        let ser = try!(serialise(student));
        Ok(ser)
    }

    fn multiple(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let students = try!(parse::<Vec<Student>>(req));
        let students = try!(students.into_iter()
            .map(|student| student.save(None, school_id, conn))
            .collect::<Result<Vec<Student>, ObsidianError>>());
        let ser = try!(serialise(students));
        Ok(ser)
    }

    try!(check_content_type(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let ser = try!(single(req, school_id, conn).or_else(|_| multiple(req, school_id, conn)));
    println!("[{}] Successfully handled students::new request", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, ser)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(Student::delete(id, school_id, conn));
    println!("[{}] Successfully handled students::delete request", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
