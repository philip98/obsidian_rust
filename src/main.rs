extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate iron;
extern crate chrono;

#[macro_use]
extern crate router;

mod models;
mod handlers;
mod middleware;

use iron::{Iron, Chain};
use middleware::PostgresConnection;
use middleware::RequestBody;

fn main() {
    let r = router!(
        students_index: get "/students" => handlers::students::index,
        students_show: get "/students/:id" => handlers::students::show,
        students_edit: put "/students/:id" => handlers::students::edit,
        students_new: post "/students" => handlers::students::new,
        students_delete: delete "/students/:id" => handlers::students::delete,

        books_index: get "/books" => handlers::books::index,
        books_show: get "/books/:id" => handlers::books::show,
        books_edit: put "/books/:id" => handlers::books::edit,
        books_new: post "/books" => handlers::books::new,
        books_delete: delete "/books/:id" => handlers::books::delete,

        aliases_index: get "/aliases" => handlers::aliases::index,
        aliases_edit: put "/aliases/:id" => handlers::aliases::edit,
        aliases_new: post "/aliases" => handlers::aliases::new,
        aliases_delete: delete "/aliases/:id" => handlers::aliases::delete,

        teachers_index: get "/teachers" => handlers::teachers::index,
        teachers_show: get "/teachers/:id" => handlers::teachers::show,
        teachers_edit: put "/teachers/:id" => handlers::teachers::edit,
        teachers_new: post "/teachers" => handlers::teachers::new,
        teachers_delete: delete "/teachers/:id" => handlers::teachers::delete,

        base_sets_new: post "/base_sets" => handlers::base_sets::new,
        base_sets_delete: delete "/base_sets/:id" => handlers::base_sets::delete,

        lendings_new: post "/lendings" => handlers::lendings::new,
        lendings_delete: delete "/lendings/:id" => handlers::lendings::delete
    );
    let connection_pool = PostgresConnection::new().unwrap();
    println!("Database connection pool initialised");
    let mut c = Chain::new(r);
    c.link_before(connection_pool);
    c.link_before(RequestBody::new());
    println!("Server up and running");
    Iron::new(c).http("localhost:3000").unwrap();
}
