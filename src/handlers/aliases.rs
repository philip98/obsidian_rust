use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, extract_id, get_includes, parse, Optionable};
use middleware::PostgresConnection;
use models::Model;
use models::aliases::Alias;

pub fn index(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = req.extensions.get::<PostgresConnection>()
        .log("PostgresConnection extension could not be found (aliases::index)")
        .and_then(|conn| json::encode(&Alias::find_all(conn, &get_includes(req)))
            .log("Serialising vector of aliases (aliases::index)")) {
        println!("[{}] Successfully handled aliases::index", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = extract_id(req)
        .and_then(|id| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (aliases::show)")
            .and_then(|conn| Alias::find_id(id, conn, &get_includes(req))))
        .and_then(|alias| alias.to_str()) {
        println!("[{}] Successfully handled aliases::show", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Alias>(req)
            .and_then(|alias| req.extensions.get::<PostgresConnection>()
                .log("PostgresConnection extension could not be found (aliases::edit)")
                .and_then(|conn| alias.save(Some(id), conn))))
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
        .and_then(|alias| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (aliases::new)")
            .and_then(|conn| alias.save(None, conn)))
        .and_then(|alias| alias.to_str()) {
        println!("[{}] Successfully handled aliases::new", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (aliases::delete)")
            .and_then(|conn| Alias::delete(id, conn))).is_some() {
        println!("[{}] Successfully handled aliases::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
