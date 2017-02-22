extern crate obsidian_rust;
extern crate iron;

use iron::Iron;
use obsidian_rust::routes::get_chain;

fn main() {
    let c = get_chain();
    println!("Server up and running");
    Iron::new(c).http("localhost:3000").unwrap();
}
