use chrono::UTC;
use iron::{IronResult, Response, Request};

use handlers::{check_content_type, get_db, get_id, get_includes, get_school_id, parse};
use models::Model;
use models::teachers::Teacher;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let teachers = try!(Teacher::find_all(school_id, conn, &includes));
    println!("[{}] Successfully handled teachers::index (include={:?})", UTC::now().format("%FT%T%:z"),
        &includes);
    respond_with!(Ok, teachers)
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let teacher = try!(Teacher::find_id(id, school_id, conn, &includes));
    println!("[{}] Successfully handled teachers::show (include={:?})", UTC::now().format("%FT%T%:z"),
        &includes);
    respond_with!(Ok, teacher)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let teacher = try!(parse::<Teacher>(req));
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let teacher = try!(teacher.save(Some(id), school_id, conn));
    println!("[{}] Successfully handled teachers::edit", UTC::now().format("%FT%T%:z"));
    respond_with!(Ok, teacher)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let teacher = try!(parse::<Teacher>(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let teacher = try!(teacher.save(None, school_id, conn));
    println!("[{}] Successfully handled teachers::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, teacher)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(Teacher::delete(id, school_id, conn));
    println!("[{}] Successfully handled teachers::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
