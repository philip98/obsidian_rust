use chrono::UTC;
use iron::{IronResult, Request, Response};

use handlers::{check_content_type, get_db, get_id, get_includes, get_school_id, parse};
use models::Model;
use models::aliases::Alias;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let conn = get_db(req);
    let school_id = get_school_id(req);
    let aliases = try!(Alias::find_all(school_id, conn, &get_includes(req)));
    println!("[{}] Successfully handled aliases::index", UTC::now().format("%FT%T%:z"));
    respond_with!(Ok, aliases)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let alias = try!(parse::<Alias>(req));
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let alias = try!(alias.save(Some(id), school_id, conn));
    println!("[{}] Successfully handled aliases::edit", UTC::now().format("%FT%T%:z"));
    respond_with!(Ok, alias)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let alias = try!(parse::<Alias>(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let alias = try!(alias.save(None, school_id, conn));
    println!("[{}] Successfully handled aliases::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, alias)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(Alias::delete(id, school_id, conn));
    println!("[{}] Successfully handled aliases::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
