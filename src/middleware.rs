use iron::{BeforeMiddleware, IronResult, IronError, Request};
use iron::status::Status;
use iron::typemap::Key;
use r2d2::{Pool, Config, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;

pub struct PostgresConnection {
    pool: Pool<PostgresConnectionManager>
}

impl PostgresConnection {
    pub fn new() -> Option<Self> {
        env::var("DATABASE_URL").ok()
            .and_then(|db_url| PostgresConnectionManager::new(db_url, TlsMode::None).ok())
            .and_then(|conn_mgr| Pool::new(Config::default(), conn_mgr).ok())
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

/*    fn catch(&self, _: &mut Request, _: IronError) -> IronResult<()> {
        Ok()
    }*/
}
