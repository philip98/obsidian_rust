use iron::{Request, IronResult, Response};
use iron::status::Status;
use router::Router;
use rustc_serialize::json;
use std::str::FromStr;

use models::students::Student;
use middleware::PostgresConnection;

pub fn index(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = req.extensions.get::<PostgresConnection>()
            .and_then(|conn| json::encode(&Student::find_all(conn)).ok()) {
        Ok(Response::with((Status::Ok, ser)))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = req.extensions.get::<Router>()
            .and_then(|params| params.find("id"))
            .and_then(|id| usize::from_str(id).ok())
            .and_then(|id| req.extensions.get::<PostgresConnection>()
                .map(|conn| (id, conn)))
            .and_then(|(id, conn)| Student::find_id(id, conn))
            .and_then(|student| json::encode(&student).ok()) {
        Ok(Response::with((Status::Ok, ser)))
    } else {
        Ok(Response::with(Status::NotFound))
    }
}
