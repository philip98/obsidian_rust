use chrono::UTC;
use iron::{IronResult, Request, Response};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status::Status;
use rustc_serialize::json;

use handlers::{check_content_type, get_body, get_db, get_school_id, Optionable};
use middleware::BasicAuthenticate;
use models::schools::{AuthData, NameChange, PasswordChange, Deletion};
use models::sessions::AuthToken;

pub fn edit(req: &mut Request) -> IronResult<Response> {
    if check_content_type(req)
        .and_then(|_| get_school_id(req))
        .and_then(|id| get_body(req)
            .and_then(|body| json::decode::<PasswordChange>(body).log("Parsing PasswordChange (schools::edit)"))
            .and_then(|pw_change| get_db(req)
                .and_then(|conn| pw_change.perform(id, conn)))).is_some() {
        println!("[{}] Successfully handled password change", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else if check_content_type(req)
        .and_then(|_| get_school_id(req))
        .and_then(|id| get_body(req)
            .and_then(|body| json::decode::<NameChange>(body).log("Parsing NameChange (schools::edit)"))
            .and_then(|name_change| get_db(req)
                .and_then(|conn| name_change.perform(id, conn)))).is_some() {
        println!("[{}] Successfully handled name change", UTC::now().format("%FT%T%:z"));
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with((Status::Unauthorized, Header(BasicAuthenticate("Token with secret".to_string())))))
    }
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    if let Some(ser) = check_content_type(req)
        .and_then(|_| get_body(req))
        .and_then(|body| json::decode::<AuthData>(body).log("Parsing AuthData (schools::new)"))
        .and_then(|auth_data| get_db(req)
            .and_then(|conn| auth_data.save(conn)
                .and_then(|id| AuthToken::new(id, conn))))
        .and_then(|token| json::encode(&token).log("Serialising AuthToken (schools::new)")) {
        Ok(Response::with((Status::Created, ser, Header(ContentType::json()))))
    } else {
        Ok(Response::with(Status::BadRequest))
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    if get_school_id(req)
        .and_then(|id| get_body(req)
            .and_then(|body| json::decode::<Deletion>(body).log("Parsing Deletion (schools::delete)"))
            .and_then(|deletion| get_db(req)
                .and_then(|conn| deletion.perform(id, conn)))).is_some() {
        Ok(Response::with(Status::NoContent))
    } else {
        Ok(Response::with((Status::Unauthorized, Header(BasicAuthenticate("Token with secret".to_string())))))
    }
}
