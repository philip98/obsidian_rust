use iron::{Request, IronResult, Response};
use iron::status::Status;
use iron::modifiers::Header;
use iron::headers::ContentType;
use rustc_serialize::json;

use models::Model;
use models::students::Student;
use middleware::PostgresConnection;
use super::{check_content_type, extract_id, parse};

pub fn index(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = req.extensions.get::<PostgresConnection>()
            .and_then(|conn| json::encode(&Student::find_all(conn)).ok()) {
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = extract_id(req)
            .and_then(|id| req.extensions.get::<PostgresConnection>()
                .and_then(|conn| Student::find_id(id, conn)))
            .and_then(|student| student.to_str()) {
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| {println!("Got 'edit' request; Content-Type is json"); extract_id(req)})
        .and_then(|id| {println!("Extracted id"); parse::<Student>(req)
            .and_then(|student| {println!("Parsed student"); req.extensions.get::<PostgresConnection>()
                .and_then(|conn| {println!("Got DB Connection"); student.save(Some(id), conn)})})})
        .and_then(|student| {println!("Saved student"); student.to_str()}) {
        println!("Successfully converted student to string");
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| {println!("Got 'new' request; Content-Type is json"); parse::<Student>(req)})
        .and_then(|student| {println!("Parsed student"); req.extensions.get::<PostgresConnection>()
            .and_then(|conn| {println!("Got DB connection"); student.save(None, conn)})})
        .and_then(|student| {println!("Successfully saved student"); student.to_str()}) {
        println!("Successfully processed request");
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else if let Some(ser) = check_content_type(req)
        .and_then(|_| {println!("Got 'new' request; Content-Type is json"); Student::parse_many(req)})
        .and_then(|students| {println!("Parsed students"); req.extensions.get::<PostgresConnection>()
            .map(|conn| {println!("Got DB connection"); students.into_iter()
                .filter_map(|student| student.save(None, conn))
                .collect::<Vec<Student>>()})})
        .and_then(|students| {println!("Saved students, length: {}", students.len()); json::encode(&students).ok()}) {
        println!("Successfully parsed request");
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| req.extensions.get::<PostgresConnection>()
            .and_then(|conn| Student::delete(id, conn))).is_some() {
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
