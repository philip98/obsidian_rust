use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::UTC;
use chrono::duration::Duration;
use iron::headers::{Authorization, Basic};
use postgres::Connection;
use rand::{thread_rng, Rng};
use std::str::FromStr;

use handlers::Optionable;

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
    pub fn from_header(data: &Authorization<Basic>) -> Option<AuthToken> {
        data.password.as_ref().log("No secret provided (AuthToken::from_header")
            .and_then(|secret| usize::from_str(&data.username).log("Token ID not integer (AuthToken::from_header)")
                .map(|token_id| AuthToken{token_id: token_id, secret: secret.clone()}))
    }

    pub fn new(id: usize, conn: &Connection) -> Option<AuthToken> {
        let token_id = thread_rng().gen::<u32>() as usize;
        let secret = thread_rng().gen_iter::<char>().take(24).collect::<String>();
        let now = UTC::now();
        conn.prepare_cached(DELETE_TOKENS).log("Preparing DELETE authentication_tokens query (AuthTokens::new)")
            .and_then(|stmt| stmt.execute(&[&(now - Duration::days(1))])
                .log("Executing DELETE authentication_tokens query (AuthTokens::new)"));
        hash(&secret, DEFAULT_COST).log("Hashing secret (AuthToken::new)")
            .and_then(|hashed_secret| conn.prepare_cached(INSERT_TOKEN)
                .log("Preparing INSERT authentication_tokens query (AuthToken::new)")
                .and_then(|stmt| stmt.query(&[&(token_id as u32), &hashed_secret, &(id as i32), &now])
                    .log("Executing INSERT authentication_tokens query (AuthToken::new)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| AuthToken{token_id: row.get::<usize, i32>(0) as usize, secret: secret})
                        .log("Id not found (AuthToken::new)"))))
    }

    pub fn verify(&self, conn: &Connection) -> Option<usize> {
        conn.prepare_cached(DELETE_TOKENS).log("Preparing DELETE authentication_tokens query (AuthTokens::verify)")
            .and_then(|stmt| stmt.execute(&[&(UTC::now() - Duration::days(1))])
                .log("Executing DELETE authentication_tokens query (AuthTokens::verify)"));
        conn.prepare_cached(QUERY_TOKENS).log("Preparing SELECT authentication_tokens query (AuthToken::verify)")
            .and_then(|stmt| stmt.query(&[&(self.token_id as i32)])
                .log("Executing SELECT authentication_tokens query (AuthToken::verify)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .and_then(|row| if verify(&self.secret, &row.get::<usize, String>(0)).unwrap_or(false) {
                        Some(row.get::<usize, i32>(1) as usize)
                    } else {
                        None
                    }.log("Wrong secret (AuthToken::verify)"))
                    .log("Token not found (AuthToken::verify)")))
    }

    pub fn verify_and_delete(&self, conn: &Connection) -> Option<()> {
        conn.prepare_cached(QUERY_TOKENS).log("Preparing SELECT authentication_tokens query (AuthToken::verify_and_delete)")
            .and_then(|stmt| stmt.query(&[&(self.token_id as i32)])
                .log("Executing SELECT authentication_tokens query (AuthToken::verify_and_delete)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .and_then(|row| if verify(&self.secret, &row.get::<usize, String>(0)).unwrap_or(false) {
                        Some(())
                    } else {
                        None
                    }.log("Wrong secret (AuthToken::verify_and_delete)"))
                    .log("Token not found (AuthToken::verify_and_delete)")))
            .and_then(|_| conn.prepare_cached(DELETE_TOKEN)
                .log("Preparing DELETE authentication_tokens query (AuthToken::verify_and_delete)")
                .and_then(|stmt| stmt.execute(&[&(self.token_id as u32), &(UTC::now() - Duration::days(1))])
                    .log("Executing DELETE authentication_tokens query (AuthToken::verify_and_delete)")))
            .and_then(|modified| if modified >= 1 {Some(())} else {None}
                .log("Token not found (AuthToken::verify_and_delete)"))
    }
}
