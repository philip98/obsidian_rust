pub mod students;
pub mod books;
pub mod aliases;

use chrono::UTC;
use iron::Request;
use iron::headers::ContentType;
use iron::mime::{TopLevel, SubLevel, Mime};
use router::Router;
use std::str::FromStr;
use std::fmt::Debug;
use std::collections::HashSet;

use models::{Model, Includable, Includes};
use middleware::RequestBody;

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

fn parse<T: Model>(req: &Request) -> Option<T> {
    req.extensions.get::<RequestBody>().log("RequestBody extension could not be found")
        .and_then(|body| T::parse_str(&body))
}

fn get_includes(req: &Request) -> Includes {
    req.url.query()
        .log("No query string provided")
        .and_then(|query| query
            .split('&')
            .filter_map(|item|
                if item.starts_with("include=") {
                    Some(Includable::parse_str(item.trim_left_matches("include=")))
                } else {
                    None
                })
            .next()
            .log("No include parameters"))
            .unwrap_or(HashSet::new())
}
