use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, parse, Optionable};
use models::Model;
use models::lendings::Lending;
use middleware::PostgresConnection;

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Lending>(req))
        .and_then(|lending| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension not found (lendings::new)")
            .and_then(|conn| lending.save(None, conn)))
        .and_then(|lending| lending.to_str()) {
        println!("[{}] Successfully handled lendings::new (single)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else if let Some(ser) = check_content_type(req)
        .and_then(|_| Lending::parse_many(req))
        .and_then(|lendings| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension not found (lendings::new)")
            .map(|conn| lendings
                .into_iter()
                .filter_map(|lending| lending.save(None, conn))
                .collect::<Vec<Lending>>()))
        .and_then(|lendings| json::encode(&lendings)
            .log("Unable to serialise vector of lendings")) {
        println!("[{}] Successfully handled lendings::new (multiple)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension not found (lendings::delete)")
            .and_then(|conn| Lending::delete(id, conn))).is_some() {
        println!("[{}] Successfully handled lendings::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
