use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, get_db, get_includes, get_school_id, parse, Optionable};
use models::Model;
use models::books::Book;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = get_db(req)
        .and_then(|conn| get_school_id(req)
            .and_then(|school_id| json::encode(&Book::find_all(school_id, conn, &includes))
            .log("Serialising vector of Books (books::index)"))) {
        println!("[{}] Successfully handled books::index (includes={:?})", UTC::now().format("%FT%T%:z"), &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| Book::find_id(id, school_id, conn, &includes))))
        .and_then(|book| book.to_str()) {
        println!("[{}] Successfully handled books::show (includes={:?})", UTC::now().format("%FT%T%:z"), &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Book>(req)
            .and_then(|book| get_db(req)
                .and_then(|conn| get_school_id(req)
                    .and_then(|school_id| book.save(Some(id), school_id, conn)))))
        .and_then(|book| book.to_str()) {
        println!("[{}] Successfully handled books::edit", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Book>(req))
        .and_then(|book| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| book.save(None, school_id, conn))))
        .and_then(|book| book.to_str()) {
        println!("[{}] Successfully handled books::new", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| Book::delete(id, school_id, conn)))).is_some() {
        println!("[{}] Successfully handled books::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
