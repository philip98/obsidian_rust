pub mod students;

use iron::Request;
use iron::headers::ContentType;
use iron::mime::{TopLevel, SubLevel, Mime};
use router::Router;
use std::str::FromStr;

use models::Model;
use middleware::RequestBody;

fn check_content_type(req: &Request) -> Option<()> {
    req.headers.get::<ContentType>()
        .and_then(|ctype| match **ctype {
            Mime(TopLevel::Application, SubLevel::Json, _) => Some(()),
            _ => None
        })
}

fn extract_id(req: &Request) -> Option<usize> {
    req.extensions.get::<Router>()
        .and_then(|params| params.find("id"))
        .and_then(|id| usize::from_str(&id).ok())
}

fn parse<T: Model>(req: &Request) -> Option<T> {
    req.extensions.get::<RequestBody>()
        .and_then(|body| T::parse_str(&body))
}
