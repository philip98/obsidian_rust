use iron::{BeforeMiddleware, IronResult, Request};
use iron::error::{HttpError, IronError};
use iron::headers::{Authorization, Basic, Header, HeaderFormat};
use iron::modifiers::{Header as MHeader};
use iron::status::Status;
use iron::typemap::Key;
use r2d2::{Pool, Config, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter, Error as FError};
use std::io::Read;

use handlers::Optionable;
use models::sessions::AuthToken;

pub struct PostgresConnection {
    pool: Pool<PostgresConnectionManager>
}

impl PostgresConnection {
    pub fn new() -> Option<Self> {
        env::var("DATABASE_URL").log("Finding DB URL (PostgresConnection::new)")
            .and_then(|db_url| PostgresConnectionManager::new(db_url, TlsMode::None)
                .log("Initialising PostgresConnectionManager (PostgresConnection::new)"))
            .and_then(|conn_mgr| Pool::new(Config::default(), conn_mgr)
                .log("Initialising connection pool (PostgresConnection::new)"))
            .map(|pool| PostgresConnection{
                pool: pool
            })
    }
}

impl Key for PostgresConnection {
    type Value = PooledConnection<PostgresConnectionManager>;
}

impl BeforeMiddleware for PostgresConnection {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        self.pool.get()
            .map_err(|err| IronError::new(err, Status::InternalServerError))
            .map(|conn| {req.extensions.insert::<PostgresConnection>(conn);})
    }
}

pub struct RequestBody;

impl RequestBody {
    pub fn new() -> RequestBody {
        RequestBody{}
    }
}

impl Key for RequestBody {
    type Value = String;
}

impl BeforeMiddleware for RequestBody {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf)
            .map(|_| {req.extensions.insert::<RequestBody>(buf);})
            .map_err(|err| IronError::new(err, Status::BadRequest))
    }
}

#[derive(Debug)]
pub struct SchoolID;

impl SchoolID {
    pub fn new() -> SchoolID {
        SchoolID{}
    }
}

impl Key for SchoolID {
    type Value = usize;
}

impl BeforeMiddleware for SchoolID {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.headers.get::<Authorization<Basic>>()
            .log("No authentication header provided")
            .and_then(|header| AuthToken::from_header(header))
            .and_then(|token| req.extensions.get::<PostgresConnection>()
                .log("PostgresConnection not found (SchoolID::before)")
                .and_then(|conn| token.verify(conn)))
        .map(|school_id| {req.extensions.insert::<Self>(school_id); Ok(())})
        .unwrap_or(Err(IronError::new(SchoolID{}, (Status::Unauthorized,
            MHeader(BasicAuthenticate("Token with secret".to_string()))))))
    }
}

impl Error for SchoolID {
    fn description(&self) -> &'static str {
        "Unauthorized"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for SchoolID {
    fn fmt(&self, _: &mut Formatter) -> Result<(), FError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BasicAuthenticate(pub String);

impl Header for BasicAuthenticate {
    fn header_name() -> &'static str {
        "WWW-Authenticate"
    }

    fn parse_header(_: &[Vec<u8>]) -> Result<Self, HttpError> {
        Err(HttpError::Header)
    }
}

impl HeaderFormat for BasicAuthenticate {
    fn fmt_header(&self, f: &mut Formatter) -> Result<(), FError> {
        let BasicAuthenticate(ref a) = *self;
        write!(f, "Basic realm=\"{}\"", a)
    }
}
