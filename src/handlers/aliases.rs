use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, get_db, get_includes, get_school_id, parse, Optionable};
use models::Model;
use models::aliases::Alias;

pub fn index(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = get_db(req)
        .and_then(|conn| get_school_id(req)
            .and_then(|school_id| json::encode(&Alias::find_all(school_id, conn, &get_includes(req)))
                .log("Serialising vector of aliases (aliases::index)"))) {
        println!("[{}] Successfully handled aliases::index", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Alias>(req)
            .and_then(|alias| get_db(req)
                .and_then(|conn| get_school_id(req)
                    .and_then(|school_id| alias.save(Some(id), school_id, conn)))))
        .and_then(|alias| alias.to_str()) {
        println!("[{}] Successfully handled aliases::edit", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Alias>(req))
        .and_then(|alias| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| alias.save(None, school_id, conn))))
        .and_then(|alias| alias.to_str()) {
        println!("[{}] Successfully handled aliases::new", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| Alias::delete(id, school_id, conn)))).is_some() {
        println!("[{}] Successfully handled aliases::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
