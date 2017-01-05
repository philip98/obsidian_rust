use chrono::UTC;
use iron::{IronResult, Response, Request};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, get_db, get_includes, parse, Optionable};
use models::Model;
use models::teachers::Teacher;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = get_db(req)
        .and_then(|conn| json::encode(&Teacher::find_all(conn, &includes))
            .log("Serialising vector of teachers (teachers::index)")) {
        println!("[{}] Successfully handled teachers::index (include={:?})", UTC::now().format("%FT%T%:z"),
            &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| Teacher::find_id(id, conn, &includes)))
        .and_then(|teacher| teacher.to_str()) {
        println!("[{}] Successfully handled teachers::show (include={:?})", UTC::now().format("%FT%T%:z"),
            &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Teacher>(req)
            .and_then(|teacher| get_db(req)
                .and_then(|conn| teacher.save(Some(id), conn))))
        .and_then(|teacher| teacher.to_str()) {
        println!("[{}] Successfully handled teachers::edit", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Teacher>(req))
        .and_then(|teacher| get_db(req)
            .and_then(|conn| teacher.save(None, conn)))
        .and_then(|teacher| teacher.to_str()) {
        println!("[{}] Successfully handled teachers::new", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| Teacher::delete(id, conn))).is_some() {
        println!("[{}] Successfully handled teachers::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
