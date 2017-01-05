use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, get_db, parse, Optionable};
use models::Model;
use models::base_sets::BaseSet;

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<BaseSet>(req))
        .and_then(|base_set| get_db(req)
            .and_then(|conn| base_set.save(None, conn)))
        .and_then(|base_set| base_set.to_str()) {
        println!("[{}] Successfully handled base_sets::new (single)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else if let Some(ser) = check_content_type(req)
        .and_then(|_| BaseSet::parse_many(req))
        .and_then(|base_sets| get_db(req)
            .map(|conn| base_sets
                .into_iter()
                .filter_map(|base_set| base_set.save(None, conn))
                .collect::<Vec<BaseSet>>()))
        .and_then(|base_sets| json::encode(&base_sets)
            .log("Unable to serialise vector of base sets")) {
        println!("[{}] Successfully handled base_sets::new (multiple)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| BaseSet::delete(id, conn))).is_some() {
        println!("[{}] Successfully handled base_sets::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
