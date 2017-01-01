use chrono::UTC;
use iron::{Request, IronResult, Response};
use iron::status::Status;
use iron::modifiers::Header;
use iron::headers::ContentType;
use rustc_serialize::json;

use models::Model;
use models::students::Student;
use middleware::PostgresConnection;
use super::{check_content_type, extract_id, parse, get_includes, Optionable};

pub fn index(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (students::index)")
            .and_then(|conn| json::encode(&Student::find_all(conn, &includes))
                .log("Serialising vector of Students (students::index)")) {
        println!("[{}] Successfully handled students::index request (include={:?})", UTC::now(), &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = extract_id(req)
            .and_then(|id| req.extensions.get::<PostgresConnection>()
                .log("PostgresConnection extension could not be found (students::show)")
                .and_then(|conn| Student::find_id(id, conn, &includes)))
            .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::show request (include={:?})", UTC::now(), &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Student>(req)
            .and_then(|student| req.extensions.get::<PostgresConnection>()
                .log("PostgresConnection extension could not be found (students::edit)")
                .and_then(|conn| student.save(Some(id), conn))))
        .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::edit request", UTC::now());
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Student>(req))
        .and_then(|student| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (students::new)")
            .and_then(|conn| student.save(None, conn)))
        .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::new request (single student)", UTC::now());
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else if let Some(ser) = check_content_type(req)
        .and_then(|_| Student::parse_many(req))
        .and_then(|students| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (students::new)")
            .map(|conn| students.into_iter()
                .filter_map(|student| student.save(None, conn))
                .collect::<Vec<Student>>()))
        .and_then(|students| json::encode(&students)
            .log("Serialising vector of Students (students::new)")) {
        println!("[{}] Successfully handled students::new request (multiple students)", UTC::now());
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| req.extensions.get::<PostgresConnection>()
            .log("PostgresConnection extension could not be found (students::delete)")
            .and_then(|conn| Student::delete(id, conn))).is_some() {
        println!("[{}] Successfully handled students::delete request", UTC::now());
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
