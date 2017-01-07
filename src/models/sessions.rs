use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::UTC;
use chrono::duration::Duration;
use iron::headers::{Authorization, Basic};
use postgres::Connection;
use rand::{thread_rng, Rng};
use std::str::FromStr;

use error::{ObsidianError, ReqError};

const INSERT_TOKEN: &'static str = "INSERT INTO authentication_tokens (id, hashed_secret, school_id, created_at)
VALUES ($1, $2, $3, $4) RETURNING id";
const QUERY_TOKENS: &'static str = "SELECT hashed_secret, school_id FROM authentication_tokens WHERE id=$1";
const DELETE_TOKEN: &'static str = "DELETE FROM authentication_tokens WHERE id=$1 OR created_at < $2";
const DELETE_TOKENS: &'static str = "DELETE FROM authentication_tokens WHERE created_at < $1";

#[derive(RustcEncodable)]
pub struct AuthToken {
    token_id: usize,
    secret: String
}

impl AuthToken {
    pub fn from_header(data: &Authorization<Basic>) -> Result<AuthToken, ObsidianError> {
        let secret = try!(data.password.clone().ok_or(ObsidianError::from(ReqError::NoAuth)));
        let token_id = try!(usize::from_str(&data.username).map_err(|_| ObsidianError::from(ReqError::NoAuth)));
        Ok(AuthToken{token_id: token_id, secret: secret})
    }

    pub fn new(id: usize, conn: &Connection) -> Result<AuthToken, ObsidianError> {
        let token_id = thread_rng().gen::<u32>() as usize;
        let secret = thread_rng().gen_iter::<char>().take(24).collect::<String>();
        let now = UTC::now();
        let stmt = try!(conn.prepare_cached(DELETE_TOKENS));
        try!(stmt.execute(&[&(now - Duration::days(1))]));
        let hashed_secret = try!(hash(&secret, DEFAULT_COST));
        let stmt2 = try!(conn.prepare_cached(INSERT_TOKEN));
        let rows = try!(stmt2.query(&[&(token_id as u32), &hashed_secret, &(id as i32), &now]));
        let row = rows.iter().next().unwrap();
        Ok(AuthToken{
            token_id: row.get::<usize, i32>(0) as usize,
            secret: secret
        })
    }

    pub fn verify(&self, conn: &Connection) -> Result<usize, ObsidianError> {
        let stmt = try!(conn.prepare_cached(DELETE_TOKENS));
        try!(stmt.execute(&[&(UTC::now() - Duration::days(1))]));
        let stmt2 = try!(conn.prepare_cached(QUERY_TOKENS));
        let rows = try!(stmt2.query(&[&(self.token_id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::from(ReqError::NoAuth)));
        if verify(&self.secret, &row.get::<usize, String>(0)).unwrap_or(false) {
            Ok(row.get::<usize, i32>(1) as usize)
        } else {
            Err(ObsidianError::from(ReqError::NoAuth))
        }
    }

    pub fn verify_and_delete(&self, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_TOKENS));
        let rows = try!(stmt.query(&[&(self.token_id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::from(ReqError::NoAuth)));
        try!(if verify(&self.secret, &row.get::<usize, String>(0)).unwrap_or(false) {
            Ok(())
        } else {
            Err(ObsidianError::from(ReqError::NoAuth))
        });
        let stmt2 = try!(conn.prepare_cached(DELETE_TOKEN));
        let modified = try!(stmt2.execute(&[&(self.token_id as u32),
            &(UTC::now() - Duration::days(1))]));
        if modified >= 1 {
            Ok(())
        } else {
            Err(ObsidianError::from(ReqError::NoAuth))
        }
    }
}
