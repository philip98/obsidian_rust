use iron::{BeforeMiddleware, IronResult, Request};
use iron::headers::{Authorization, Basic};
use iron::typemap::Key;
use r2d2::{Pool, Config, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;
use std::io::Read;

use error::{ObsidianError, ReqError};
use models::sessions::AuthToken;

pub struct PostgresConnection {
    pool: Pool<PostgresConnectionManager>
}

impl PostgresConnection {
    pub fn new() -> Result<PostgresConnection, ObsidianError> {
        let url = env::var("DATABASE_URL").expect("No database url provided");
        let mgr = try!(PostgresConnectionManager::new(url, TlsMode::None));
        let pool = try!(Pool::new(Config::default(), mgr));
        Ok(PostgresConnection{pool: pool})
    }
}

impl Key for PostgresConnection {
    type Value = PooledConnection<PostgresConnectionManager>;
}

impl BeforeMiddleware for PostgresConnection {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = try!(self.pool.get().map_err(|err| ObsidianError::from(err)));
        req.extensions.insert::<Self>(conn);
        Ok(())
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
        try!(req.body.read_to_string(&mut buf).map_err(ObsidianError::from));
        req.extensions.insert::<Self>(buf);
        Ok(())
    }
}

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
        let school_id = {
            let header = try!(req.headers.get::<Authorization<Basic>>().ok_or(ObsidianError::from(ReqError::NoAuth)));
            let token = try!(AuthToken::from_header(header));
            let conn = req.extensions.get::<PostgresConnection>().unwrap();
            try!(token.verify(conn))
        };
        req.extensions.insert::<Self>(school_id);
        Ok(())
    }
}
