use chrono::UTC;
use iron::{IronResult, Request, Response};
use postgres::Connection;

use error::ObsidianError;
use handlers::{check_content_type, get_db, get_id, get_school_id, parse, serialise};
use models::Model;
use models::base_sets::BaseSet;

pub fn new(req: &mut Request) -> IronResult<Response> {
    fn single(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let base_set = try!(parse::<BaseSet>(req));
        let base_set = try!(base_set.save(None, school_id, conn));
        let ser = try!(serialise(base_set));
        Ok(ser)
    }

    fn multiple(req: &Request, school_id: usize, conn: &Connection) -> IronResult<String> {
        let base_sets = try!(parse::<Vec<BaseSet>>(req));
        let base_sets = try!(base_sets
            .into_iter()
            .map(|base_set| base_set.save(None, school_id, conn))
            .collect::<Result<Vec<BaseSet>, ObsidianError>>());
        let ser = try!(serialise(base_sets));
        Ok(ser)
    }

    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(check_content_type(req));
    let ser = try!(single(req, school_id, conn).or_else(|_| multiple(req, school_id, conn)));
    println!("[{}] Successfully handled base_sets::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, ser)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let school_id = get_school_id(req);
    let conn = get_db(req);
    try!(BaseSet::delete(id, school_id, conn));
    println!("[{}] Successfully handled base_sets::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
