use chrono::UTC;
use iron::{IronResult, Request, Response};
use postgres::Connection;

use handlers::{check_content_type, get_db, get_school_id, parse};
use models::schools::{AuthData, NameChange, PasswordChange, Deletion};
use models::sessions::AuthToken;

pub fn edit(req: &mut Request) -> IronResult<Response> {
    fn password(req: &Request, id: usize, conn: &Connection) -> IronResult<()> {
        let pw_change = try!(parse::<PasswordChange>(req));
        try!(pw_change.perform(id, conn));
        Ok(())
    }

    fn name(req: &Request, id: usize, conn: &Connection) -> IronResult<()> {
        let name_change = try!(parse::<NameChange>(req));
        try!(name_change.perform(id, conn));
        Ok(())
    }

    try!(check_content_type(req));
    let id = get_school_id(req);
    let conn = get_db(req);
    try!(password(req, id, conn).or_else(|_| name(req, id, conn)));
    println!("[{}] Successfully handled schools::edit", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let auth_data = try!(parse::<AuthData>(req));
    let conn = get_db(req);
    let id = try!(auth_data.save(conn));
    let token = try!(AuthToken::new(id, conn));
    println!("[{}] Successfully handled schools::new", UTC::now().format("%FT%T%:z"));
    respond_with!(Created, token)
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    try!(check_content_type(req));
    let deletion = try!(parse::<Deletion>(req));
    let id = get_school_id(req);
    let conn = get_db(req);
    try!(deletion.perform(id, conn));
    println!("[{}] Successfully handled schools::delete", UTC::now().format("%FT%T%:z"));
    respond_with!(NoContent)
}
