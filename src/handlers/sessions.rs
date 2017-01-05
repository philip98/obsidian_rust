use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::{Authorization, Basic, ContentType};
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, get_body, get_db, Optionable};
use middleware::BasicAuthenticate;
use models::schools::AuthData;
use models::sessions::AuthToken;

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| get_body(req))
        .and_then(|body| json::decode::<AuthData>(body).log("Parsing AuthData (sessions::new)"))
        .and_then(|auth| get_db(req)
            .and_then(|conn| auth.verify(conn)
                .and_then(|id| AuthToken::new(id, conn))))
        .and_then(|token| json::encode(&token).log("Serialising AuthToken (sessions::new)")) {
        println!("[{}] Successfully handled sessions::new", UTC::now().format("%FT%T%:z"));
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with((Status::Unauthorized, Header(BasicAuthenticate("Token with secret".to_string())))))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if req.headers.get::<Authorization<Basic>>()
        .log("No Authorization header provided")
        .and_then(|header| AuthToken::from_header(header))
        .and_then(|token| get_db(req)
            .and_then(|conn| token.verify_and_delete(conn))).is_some() {
        println!("[{}] Successfully handlede sessions::delete", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with((Status::Unauthorized, Header(BasicAuthenticate("Token with secret".to_string())))))
    }
}
