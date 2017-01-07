use chrono::UTC;
use iron::{IronResult, Request, Response};
use postgres::Connection;

use error::ObsidianError;
use handlers::{check_content_type, get_db, get_id, get_school_id, parse, serialise};
use models::Model;
use models::lendings::Lending;

pub fn new(req: &mut Request) -> IronResult<Response> {
    fn single(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let lending = try!(parse::<Lending>(req));
        let lending = try!(lending.save(None, school_id, conn));
        let ser = try!(serialise(lending));
        Ok(ser)
    }

    fn multiple(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let lendings = try!(parse::<Vec<Lending>>(req));
        let lendings = try!(lendings
            .into_iter()
            .map(|lending| lending.save(None, school_id, conn))
            .collect::<Result<Vec<Lending>, ObsidianError>>());
        let ser = try!(serialise(lendings));
        Ok(ser)
    }

    try!(check_content_type(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    let ser = try!(single(req, school_id, conn).or_else(|_| multiple(req, school_id, conn)));
    println!("[{}] Successfully handled lendings::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, ser)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(Lending::delete(id, school_id, conn));
    println!("[{}] Successfully handled lendings::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
