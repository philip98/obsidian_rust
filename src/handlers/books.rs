use chrono::UTC;
use iron::{IronResult, Request, Response};

use handlers::{check_content_type, get_db, get_id, get_includes, get_school_id, parse};
use models::Model;
use models::books::Book;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let books = try!(Book::find_all(school_id, conn, &includes));
    println!("[{}] Successfully handled books::index (includes={:?})", UTC::now().format("%FT%T%:z"), &includes);
    respond_with!(Ok, books)
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let includes = get_includes(req);
    let book = try!(Book::find_id(id, school_id, conn, &includes));
    println!("[{}] Successfully handled books::show (includes={:?})", UTC::now().format("%FT%T%:z"), &includes);
    respond_with!(Ok, book)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let book = try!(parse::<Book>(req));
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let book = try!(book.save(Some(id), school_id, conn));
    println!("[{}] Successfully handled books::edit", UTC::now().format("%FT%T%:z"));
    respond_with!(Ok, book)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let book = try!(parse::<Book>(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let book = try!(book.save(None, school_id, conn));
    println!("[{}] Successfully handled books::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, book)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(Book::delete(id, school_id, conn));
    println!("[{}] Successfully handled books::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
