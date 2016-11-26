extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate iron;

#[macro_use]
extern crate router;

mod models;
mod handlers;
mod middleware;

use iron::{Iron, Chain};
use middleware::PostgresConnection;

fn main() {
    let r = router!(
        students_index: get "/students" => handlers::students::index,
        students_get: get "/students/:id" => handlers::students::get);
    let connection_pool = PostgresConnection::new().unwrap();
    let mut c = Chain::new(r);
    c.link_before(connection_pool);
    Iron::new(c).http("localhost:3000").unwrap();
}
