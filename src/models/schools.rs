use bcrypt::{hash, verify, DEFAULT_COST};
use postgres::Connection;

use handlers::Optionable;

const QUERY_SCHOOLS: &'static str = "SELECT id, encrypted_password FROM schools WHERE name=$1";
const QUERY_SCHOOL: &'static str = "SELECT encrypted_password FROM schools WHERE id=$1";

const INSERT_SCHOOL: &'static str = "INSERT INTO schools (name, encrypted_password) VALUES ($1, $2) RETURNING id";
const UPDATE_NAME: &'static str = "UPDATE schools SET name=$2 WHERE id=$1";
const UPDATE_PASSWORD: &'static str = "UPDATE schools SET encrypted_password=$2 WHERE id=$1";
const DELETE_SCHOOL: &'static str = "DELETE FROM schools WHERE id=$1";

#[derive(RustcDecodable)]
pub struct AuthData {
    name: String,
    password: String
}

impl AuthData {
    pub fn verify(&self, conn: &Connection) -> Option<usize> {
        conn.prepare_cached(QUERY_SCHOOLS).log("Preparing SELECT schools query (AuthData::verify)")
            .and_then(|stmt| stmt.query(&[&self.name.to_lowercase()])
                .log("Executing SELECT schools query (AuthData::verify)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .and_then(|row| if verify(&self.password, &row.get::<usize, String>(1)).unwrap_or(false) {
                        Some(row.get::<usize, i32>(0) as usize)
                    } else {
                        None
                    }.log("Wrong password (AuthData::verify"))
                    .log("School not found (AuthData::verify)")))
    }

    pub fn save(&self, conn: &Connection) -> Option<usize> {
        hash(&self.password, DEFAULT_COST).log("Hashing password (AuthData::save)")
            .and_then(|encrypted_password| conn.prepare_cached(INSERT_SCHOOL)
                .log("Preparing INSERT schools query (AuthData::save)")
                .and_then(|stmt| stmt.query(&[&self.name.to_lowercase(), &encrypted_password])
                    .log("Executing INSERT schools query (AuthData::save)")
                    .and_then(|rows| rows
                        .iter()
                        .next()
                        .map(|row| row.get::<usize,i32>(0) as usize)
                        .log("No id found (AuthData::save)"))))
    }
}

#[derive(RustcDecodable)]
pub struct PasswordChange {
    old_password: String,
    new_password: String
}

impl PasswordChange {
    pub fn perform(&self, id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(QUERY_SCHOOL).log("Preparing SELECT schools query (PasswordChange::perform)")
            .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT schools query (PasswordChange::perform)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .map(|row| if verify(&self.old_password, &row.get::<usize, String>(0)).unwrap_or(false) {
                        Some(())
                    } else {
                        None
                    }.log("Incorrect old password (PasswordChange::perform)"))
                    .log("School not found (PasswordChange::perform)")))
            .and_then(|_| hash(&self.new_password, DEFAULT_COST)
                .log("Hashing new password (PasswordChange::perform)")
                .and_then(|encrypted_password| conn.prepare_cached(UPDATE_PASSWORD)
                    .log("Preparing UPDATE schools query (PasswordChange::perform)")
                    .and_then(|stmt| stmt.execute(&[&(id as i32), &encrypted_password])
                        .log("Executing UPDATE schools query (PasswordChange::perform)"))))
            .and_then(|modified| if modified == 1 {Some(())} else {None}
                .log("School not changed (PasswordChange::perform)"))
    }
}

#[derive(RustcDecodable)]
pub struct NameChange {
    name: String
}

impl NameChange {
    pub fn perform(&self, id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(UPDATE_NAME).log("Preparing UPDATE schools query (NameChange::perform)")
            .and_then(|stmt| stmt.execute(&[&(id as i32), &self.name])
                .log("Executing UPDATE schools query (NameChange::perform)"))
            .and_then(|modified| if modified == 1 {Some(())} else {None}
                .log("School not found (NameChange::perform)"))
    }
}

#[derive(RustcDecodable)]
pub struct Deletion {
    password: String
}

impl Deletion {
    pub fn perform(&self, id: usize, conn: &Connection) -> Option<()> {
        conn.prepare_cached(QUERY_SCHOOL).log("Preparing SELECT schools query (Deletion::perform)")
            .and_then(|stmt| stmt.query(&[&(id as i32)]).log("Executing SELECT schools query (Deletion::perform)")
                .and_then(|rows| rows
                    .iter()
                    .next()
                    .map(|row| if verify(&self.password, &row.get::<usize, String>(0)).unwrap_or(false) {
                        Some(())
                    } else {
                        None
                    }.log("Incorrect old password (Deletion::perform)"))
                    .log("School not found (Deletion::perform)")))
            .and_then(|_| conn.prepare_cached(DELETE_SCHOOL)
                .log("Preparing DELETE schools query (Deletion::perform)")
                .and_then(|stmt| stmt.execute(&[&(id as i32)])
                    .log("Executing DELETE schools query (Deletion::perform)"))
                .and_then(|modified| if modified == 1 {Some(())} else {None}
                    .log("School not found (Deletion::perform)")))
    }
}
