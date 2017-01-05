extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate iron;
extern crate chrono;
extern crate rand;
extern crate bcrypt;

#[macro_use]
extern crate router;

mod models;
mod handlers;
mod middleware;

use iron::{Iron, Chain};
use middleware::{PostgresConnection, RequestBody};
use handlers::auth;

fn main() {
    let r = router!(
        students_index: get "/students" =>         auth(handlers::students::index),
        students_show: get "/students/:id" =>      auth(handlers::students::show),
        students_edit: put "/students/:id" =>      auth(handlers::students::edit),
        students_new: post "/students" =>          auth(handlers::students::new),
        students_delete: delete "/students/:id" => auth(handlers::students::delete),

        books_index: get "/books" =>         auth(handlers::books::index),
        books_show: get "/books/:id" =>      auth(handlers::books::show),
        books_edit: put "/books/:id" =>      auth(handlers::books::edit),
        books_new: post "/books" =>          auth(handlers::books::new),
        books_delete: delete "/books/:id" => auth(handlers::books::delete),

        aliases_index: get "/aliases" =>         auth(handlers::aliases::index),
        aliases_edit: put "/aliases/:id" =>      auth(handlers::aliases::edit),
        aliases_new: post "/aliases" =>          auth(handlers::aliases::new),
        aliases_delete: delete "/aliases/:id" => auth(handlers::aliases::delete),

        teachers_index: get "/teachers" =>         auth(handlers::teachers::index),
        teachers_show: get "/teachers/:id" =>      auth(handlers::teachers::show),
        teachers_edit: put "/teachers/:id" =>      auth(handlers::teachers::edit),
        teachers_new: post "/teachers" =>          auth(handlers::teachers::new),
        teachers_delete: delete "/teachers/:id" => auth(handlers::teachers::delete),

        base_sets_new: post "/base_sets" =>          auth(handlers::base_sets::new),
        base_sets_delete: delete "/base_sets/:id" => auth(handlers::base_sets::delete),

        lendings_new: post "/lendings" =>          auth(handlers::lendings::new),
        lendings_delete: delete "/lendings/:id" => auth(handlers::lendings::delete),

        schools_edit: put "/schools" =>      auth(handlers::schools::edit),
        schools_new: post "/schools" =>      handlers::schools::new,
        schools_delete: delete "/schools" => auth(handlers::schools::delete),

        sessions_new: post "/sessions" =>      handlers::sessions::new,
        sessions_delete: delete "/sessions" => handlers::sessions::delete
    );
    let connection_pool = PostgresConnection::new().unwrap();
    println!("Database connection pool initialised");
    let mut c = Chain::new(r);
    c.link_before(connection_pool);
    c.link_before(RequestBody::new());
    println!("Server up and running");
    Iron::new(c).http("localhost:3000").unwrap();
}
