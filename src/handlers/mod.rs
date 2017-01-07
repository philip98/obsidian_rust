macro_rules! respond_with {
    ($st:ident) => (
        ::std::result::Result::Ok(::iron::response::Response::with(::iron::status::Status::$st))
    );
    ($st:ident, $body:expr) => ({
        let ser = try!($crate::handlers::serialise($body));
        ::std::result::Result::Ok(::iron::response::Response::with((::iron::status::Status::$st, ser,
            ::iron::modifiers::Header(::iron::headers::ContentType::json()))))
    });
}

pub mod students;
pub mod books;
pub mod aliases;
pub mod teachers;
pub mod base_sets;
pub mod lendings;
pub mod schools;
pub mod sessions;

use iron::{Chain, Handler, Request};
use iron::headers::ContentType;
use iron::mime::{TopLevel, SubLevel, Mime};
use postgres::Connection;
use router::Router;
use std::str::FromStr;
use std::collections::HashSet;
use rustc_serialize::{Decodable, Encodable, json};

use error::{ObsidianError, ReqError};
use models::{Includable, Includes};
use middleware::{PostgresConnection, RequestBody, SchoolID};

fn check_content_type(req: &Request) -> Result<(), ObsidianError> {
    try!(req.headers.get::<ContentType>()
        .and_then(|ctype| match **ctype {
            Mime(TopLevel::Application, SubLevel::Json, _) => Some(()),
            _ => None
        }).ok_or(ReqError::WrongContentType));
    Ok(())
}

fn get_id(req: &Request) -> Result<usize, ObsidianError> {
    let router_params = req.extensions.get::<Router>().unwrap();
    let id = router_params.find("id").unwrap();
    usize::from_str(&id).map_err(|_| ObsidianError::from(ReqError::NoID))
}

fn get_body<'a>(req: &'a Request) -> &'a str {
    req.extensions.get::<RequestBody>().unwrap().as_ref()
}

fn parse<T: Decodable>(req: &Request) -> Result<T, ObsidianError> {
    json::decode::<T>(get_body(req)).map_err(ObsidianError::from)
}

fn serialise<T: Encodable>(t: T) -> Result<String, ObsidianError> {
    json::encode(&t).map_err(ObsidianError::from)
}

fn get_db<'a>(req: &'a Request) -> &'a Connection {
    &**req.extensions.get::<PostgresConnection>().unwrap()
}

fn get_includes(req: &Request) -> Includes {
    req.url.query()
        .and_then(|query| query
            .split('&')
            .filter_map(|item|
                if item.starts_with("include=") {
                    Some(Includable::parse_str(item.trim_left_matches("include=")))
                } else {
                    None
                })
            .next())
        .unwrap_or(HashSet::new())
}

fn get_school_id(req: &Request) -> usize {
    *req.extensions.get::<SchoolID>().unwrap()
}

pub fn auth<H: Handler>(h: H) -> Chain {
    let mut res = Chain::new(h);
    res.link_before(SchoolID::new());
    res
}
