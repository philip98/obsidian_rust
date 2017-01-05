use chrono::UTC;
use iron::{Request, IronResult, Response};
use iron::status::Status;
use iron::modifiers::Header;
use iron::headers::ContentType;
use rustc_serialize::json;

use models::Model;
use models::students::Student;
use super::{check_content_type, extract_id, parse, get_db, get_includes, get_school_id, Optionable};

pub fn index(req: &mut Request) -> IronResult<Response> {
    let includes = get_includes(req);
    if let Some(ser) = get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| json::encode(&Student::find_all(school_id, conn, &includes))
                    .log("Serialising vector of Students (students::index)"))) {
        println!("[{}] Successfully handled students::index request (include={:?})", UTC::now().format("%FT%T%:z"), &includes);
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
                    .and_then(|school_id| Student::find_id(id, school_id, conn, &includes))))
            .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::show request (include={:?})", UTC::now().format("%FT%T%:z"), &includes);
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| extract_id(req))
        .and_then(|id| parse::<Student>(req)
            .and_then(|student| get_db(req)
                .and_then(|conn| get_school_id(req)
                    .and_then(|school_id| student.save(Some(id), school_id, conn)))))
        .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::edit request", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Ok, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| parse::<Student>(req))
        .and_then(|student| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| student.save(None, school_id, conn))))
        .and_then(|student| student.to_str()) {
        println!("[{}] Successfully handled students::new request (single student)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else if let Some(ser) = check_content_type(req)
        .and_then(|_| Student::parse_many(req))
        .and_then(|students| get_db(req)
            .and_then(|conn| get_school_id(req)
                .map(|school_id| students.into_iter()
                    .filter_map(|student| student.save(None, school_id, conn))
                    .collect::<Vec<Student>>())))
        .and_then(|students| json::encode(&students)
            .log("Serialising vector of Students (students::new)")) {
        println!("[{}] Successfully handled students::new request (multiple students)", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if extract_id(req)
        .and_then(|id| get_db(req)
            .and_then(|conn| get_school_id(req)
                .and_then(|school_id| Student::delete(id, school_id, conn)))).is_some() {
        println!("[{}] Successfully handled students::delete request", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
