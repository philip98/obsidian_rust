use iron::{BeforeMiddleware, IronResult, IronError, Request};
use iron::status::Status;
use iron::typemap::Key;
use r2d2::{Pool, Config, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;
use std::io::Read;

use handlers::Optionable;

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
