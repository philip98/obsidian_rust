use bcrypt::{hash, verify, DEFAULT_COST};
use postgres::Connection;

use error::ObsidianError;

const QUERY_SCHOOLS: &'static str = "SELECT id, encrypted_password FROM schools WHERE name=$1";
const QUERY_SCHOOL: &'static str = "SELECT encrypted_password FROM schools WHERE id=$1";

const INSERT_SCHOOL: &'static str = "INSERT INTO schools (name, encrypted_password) VALUES ($1, $2) RETURNING id";
const UPDATE_NAME: &'static str = "UPDATE schools SET name=$2 WHERE id=$1";
const UPDATE_PASSWORD: &'static str = "UPDATE schools SET encrypted_password=$2 WHERE id=$1";
const DELETE_SCHOOL: &'static str = "DELETE FROM schools WHERE id=$1";

#[derive(RustcDecodable, Debug)]
pub struct AuthData {
    name: String,
    password: String
}

impl AuthData {
    pub fn verify(&self, conn: &Connection) -> Result<usize, ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_SCHOOLS));
        let rows = try!(stmt.query(&[&self.name.to_lowercase()]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("School")));
        if verify(&self.password, &row.get::<usize, String>(1)).unwrap_or(false) {
            Ok(row.get::<usize, i32>(0) as usize)
        } else {
            Err(ObsidianError::WrongPassword)
        }
    }

    pub fn save(&self, conn: &Connection) -> Result<usize, ObsidianError> {
        let encrypted_password = try!(hash(&self.password, DEFAULT_COST));
        let stmt = try!(conn.prepare_cached(INSERT_SCHOOL));
        let rows = try!(stmt.query(&[&self.name.to_lowercase(), &encrypted_password]));
        let row = rows.iter().next().unwrap();
        Ok(row.get::<usize, i32>(0) as usize)
    }
}

#[derive(RustcDecodable, Debug)]
pub struct PasswordChange {
    old_password: String,
    new_password: String
}

impl PasswordChange {
    pub fn perform(&self, id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_SCHOOL));
        let rows = try!(stmt.query(&[&(id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("School")));
        try!(if verify(&self.old_password, &row.get::<usize, String>(0)).unwrap_or(false) {
            Ok(())
        } else {
            Err(ObsidianError::WrongPassword)
        });
        let encrypted_password = try!(hash(&self.new_password, DEFAULT_COST));
        let stmt2 = try!(conn.prepare_cached(UPDATE_PASSWORD));
        let modified = try!(stmt2.execute(&[&(id as i32), &encrypted_password]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("School"))
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct NameChange {
    name: String
}

impl NameChange {
    pub fn perform(&self, id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(UPDATE_NAME));
        let modified = try!(stmt.execute(&[&(id as i32), &self.name]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("School"))
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Deletion {
    password: String
}

impl Deletion {
    pub fn perform(&self, id: usize, conn: &Connection) -> Result<(), ObsidianError> {
        let stmt = try!(conn.prepare_cached(QUERY_SCHOOL));
        let rows = try!(stmt.query(&[&(id as i32)]));
        let row = try!(rows.iter().next().ok_or(ObsidianError::RecordNotFound("School")));
        try!(if verify(&self.password, &row.get::<usize, String>(0)).unwrap_or(false) {
            Ok(())
        } else {
            Err(ObsidianError::WrongPassword)
        });
        let stmt2 = try!(conn.prepare_cached(DELETE_SCHOOL));
        let modified = try!(stmt2.execute(&[&(id as i32)]));
        if modified == 1 {
            Ok(())
        } else {
            Err(ObsidianError::RecordNotFound("School"))
        }
    }
}
