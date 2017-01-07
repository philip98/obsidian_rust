use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::{Authorization, Basic};

use error::{ObsidianError, ReqError};
use handlers::{check_content_type, get_db, parse};
use models::schools::AuthData;
use models::sessions::AuthToken;

pub fn new(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let auth = try!(parse::<AuthData>(req));
    let conn = get_db(req);
    let id = try!(auth.verify(conn));
    let token = try!(AuthToken::new(id, conn));
    println!("[{}] Successfully handled sessions::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, token)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let header = try!(req.headers.get::<Authorization<Basic>>().ok_or(ObsidianError::from(ReqError::NoAuth)));
    let token = try!(AuthToken::from_header(header));
    let conn = get_db(req);
    try!(token.verify_and_delete(conn));
    println!("[{}] Successfully handlede sessions::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
