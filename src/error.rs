use iron::error::{IronError, HttpError};
use iron::headers::{Header as THeader, HeaderFormat};
use iron::modifiers::Header;
use iron::status::Status;
use std::error::Error;
use std::fmt::{Display, Error as FError, Formatter};
use std::convert::From;

#[derive(Debug)]
pub enum ReqError {
    NoID,
    WrongContentType,
    NoAuth
}

#[derive(Debug)]
pub enum ObsidianError {
    RequestError(::error::ReqError),
    PostgresError(::postgres::error::Error),
    ConnectionError(::postgres::error::ConnectError),
    GetConnError(::r2d2::GetTimeout),
    RecordNotFound(&'static str),
    PoolError(::r2d2::InitializationError),
    ParseError(::rustc_serialize::json::DecoderError),
    SerializeError(::rustc_serialize::json::EncoderError),
    IoError(::std::io::Error),
    IncludeNotSupported(::models::Includable),
    WrongPassword,
    BCryptError(::bcrypt::BcryptError)
}

macro_rules! impl_oerr {
    ($er:path, $it:ident) => (impl ::std::convert::From<$er> for $crate::error::ObsidianError {
        fn from(err: $er) -> Self {
            $crate::error::ObsidianError::$it(err)
        }
    });
}

impl_oerr!(::error::ReqError, RequestError);
impl_oerr!(::postgres::error::Error, PostgresError);
impl_oerr!(::postgres::error::ConnectError, ConnectionError);
impl_oerr!(::r2d2::InitializationError, PoolError);
impl_oerr!(::rustc_serialize::json::DecoderError, ParseError);
impl_oerr!(::rustc_serialize::json::EncoderError, SerializeError);
impl_oerr!(::r2d2::GetTimeout, GetConnError);
impl_oerr!(::std::io::Error, IoError);
impl_oerr!(::models::Includable, IncludeNotSupported);
impl_oerr!(::bcrypt::BcryptError, BCryptError);

impl From<ObsidianError> for IronError {
    fn from(err: ObsidianError) -> IronError {
        let just_status = match err {
            ObsidianError::PostgresError(ref inner) => {
                println!("PostgresError: {:?}", inner);
                Some(Status::InternalServerError)
            },
            ObsidianError::ConnectionError(ref inner) => {
                println!("ConnectError: {:?}", inner);
                Some(Status::InternalServerError)
            },
            ObsidianError::GetConnError(ref inner) => {
                println!("GetTimeout: {:?}", &inner);
                Some(Status::InternalServerError)
            },
            ObsidianError::RecordNotFound(name) if  name != "School" => {
                println!("{} not found", name);
                Some(Status::NotFound)
            },
            ObsidianError::PoolError(ref inner) => {
                println!("R2D2 Initialization Error: {:?}", inner);
                Some(Status::InternalServerError)
            },
            ObsidianError::ParseError(ref inner) => {
                println!("Parser error: {:?}", inner);
                Some(Status::BadRequest)
            },
            ObsidianError::SerializeError(ref inner) => {
                println!("Serialisation error: {:?}", inner);
                unreachable!()
            },
            ObsidianError::IoError(ref inner) => {
                println!("Io error: {:?}", inner);
                Some(Status::BadRequest)
            },
            _ => {None}
        };

        let status_and_header = match err {
            ObsidianError::RequestError(ReqError::NoAuth) => {
                println!("No authentication token provided");
                Some((Status::Unauthorized, Header(BasicAuthenticate("Token and secret".to_string()))))
            },
            ObsidianError::WrongPassword => {
                println!("Wrong password");
                Some((Status::Unauthorized, Header(BasicAuthenticate("Token and secret".to_string()))))
            },
            ObsidianError::BCryptError(ref inner) => {
                println!("BCrypt error: {:?}", inner);
                Some((Status::Unauthorized, Header(BasicAuthenticate("Token and secret".to_string()))))
            },
            ObsidianError::RecordNotFound("School") => {
                println!("School not found");
                Some((Status::Unauthorized, Header(BasicAuthenticate("Token and secret".to_string()))))
            }
            _ => {None}
        };

        let header_and_text = match err {
            ObsidianError::IncludeNotSupported(ref inner) => {
                println!("{:?} not supported as include", inner);
                Some((Status::BadRequest, "The relation to be included is not supported by this route"))
            },
            ObsidianError::RequestError(ReqError::NoID) => {
                println!("Wrongly typed id");
                Some((Status::BadRequest, "The id needs to be an integer"))
            },
            ObsidianError::RequestError(ReqError::WrongContentType) => {
                println!("Wrong Content-Type");
                Some((Status::BadRequest, "Content-Type need to be application/json"))
            },
            _ => {None}
        };

        if let Some(st) = just_status {
            IronError::new(err, st)
        } else if let Some(st) = status_and_header {
            IronError::new(err, st)
        } else if let Some(st) = header_and_text {
            IronError::new(err, st)
        } else {
            unreachable!()
        }
    }
}

impl Error for ObsidianError {
    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ObsidianError::RequestError(_) |
            ObsidianError::RecordNotFound(_) |
            ObsidianError::IncludeNotSupported(_) |
            ObsidianError::WrongPassword => {None},
            ObsidianError::PostgresError(ref a) => {Some(a)},
            ObsidianError::ConnectionError(ref a) => {Some(a)},
            ObsidianError::GetConnError(ref a) => {Some(a)},
            ObsidianError::PoolError(ref a) => {Some(a)},
            ObsidianError::ParseError(ref a) => {Some(a)},
            ObsidianError::SerializeError(ref a) => {Some(a)},
            ObsidianError::IoError(ref a) => {Some(a)},
            ObsidianError::BCryptError(ref a) => {Some(a)}
        }
    }
}

impl Display for ObsidianError {
    fn fmt(&self, _: &mut Formatter) -> Result<(), FError> {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
struct BasicAuthenticate(String);

impl THeader for BasicAuthenticate {
    fn header_name() -> &'static str {
        "WWW-Authenticate"
    }

    fn parse_header(_: &[Vec<u8>]) -> Result<Self, HttpError> {
        unreachable!()
    }
}

impl HeaderFormat for BasicAuthenticate {
    fn fmt_header(&self, f: &mut Formatter) -> Result<(), FError> {
        let BasicAuthenticate(ref a) = *self;
        write!(f, "Basic: realm=\"{}\"", a)
    }
}
