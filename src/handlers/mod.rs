pub mod students;
pub mod books;
pub mod aliases;
pub mod teachers;
pub mod base_sets;
pub mod lendings;
pub mod schools;
pub mod sessions;

use chrono::UTC;
use iron::{Chain, Handler, Request};
use iron::headers::ContentType;
use iron::mime::{TopLevel, SubLevel, Mime};
use postgres::Connection;
use router::Router;
use std::str::FromStr;
use std::fmt::Debug;
use std::collections::HashSet;

use models::{Model, Includable, Includes};
use middleware::{PostgresConnection, RequestBody, SchoolID};

pub trait Optionable<T>: Sized {
    fn log_to_option(self, ctxt: Option<&str>) -> Option<T>;
    fn to_option(self) -> Option<T> {
        self.log_to_option(None)
    }
    fn log(self, ctxt: &str) -> Option<T> {
        self.log_to_option(Some(ctxt))
    }
}

impl<T,E> Optionable<T> for Result<T, E> where E: Debug {
    fn log_to_option(self, ctxt: Option<&str>) -> Option<T> {
        match self {
            Ok(x) => {
                Some(x)
            },
            Err(e) => {
                if let Some(ctxt) = ctxt {
                    println!("[{}] {}: {:?}", UTC::now().format("%FT%T%:z"), ctxt, e);
                } else {
                    println!("[{}] Error: {:?}", UTC::now().format("%FT%T%:z"), e);
                }
                None
            }
        }
    }
}

impl<T> Optionable<T> for Option<T> {
    fn log_to_option(self, ctxt: Option<&str>) -> Option<T> {
        match self {
            None => {
                if let Some(ctxt) = ctxt {
                    println!("[{}] {}", UTC::now().format("%FT%T%:z"), ctxt);
                }
                None
            },
            Some(x) => {
                Some(x)
            }
        }
    }
}

fn check_content_type(req: &Request) -> Option<()> {
    req.headers.get::<ContentType>().log("Content-Type header could not be found (check_content_type)")
        .and_then(|ctype| match **ctype {
            Mime(TopLevel::Application, SubLevel::Json, _) => Some(()),
            _ => None
        }.log("Content-Type is not 'application/json' (check_content_type)"))
}

fn extract_id(req: &Request) -> Option<usize> {
    req.extensions.get::<Router>().log("Router extension could not be found (extract_id)")
        .and_then(|params| params.find("id").log("'id' param could not be found (extract_id)"))
        .and_then(|id| usize::from_str(&id).log("Conversion of 'id' param to usize (extract_id)"))
}

fn get_body<'a>(req: &'a Request) -> Option<&'a str> {
    req.extensions.get::<RequestBody>().log("RequestBody could not be found (get_body)")
        .map(|body| body.as_ref())
}

fn parse<T: Model>(req: &Request) -> Option<T> {
    get_body(req).and_then(|body| T::parse_str(&body))
}

fn get_db<'a>(req: &'a Request) -> Option<&'a Connection> {
    req.extensions.get::<PostgresConnection>().log("PostgresConnection could not be found (get_db)")
        .map(|conn| &**conn)
}

fn get_includes(req: &Request) -> Includes {
    req.url.query()
        .log("No query string provided (get_includes)")
        .and_then(|query| query
            .split('&')
            .filter_map(|item|
                if item.starts_with("include=") {
                    Some(Includable::parse_str(item.trim_left_matches("include=")))
                } else {
                    None
                })
            .next()
            .log("No include parameters (get_includes)"))
            .unwrap_or(HashSet::new())
}

fn get_school_id(req: &Request) -> Option<usize> {
    req.extensions.get::<SchoolID>().log("School id not found (get_school_id)")
        .map(|id| *id)
}

pub fn auth<H: Handler>(h: H) -> Chain {
    let mut res = Chain::new(h);
    res.link_before(SchoolID::new());
    res
}
