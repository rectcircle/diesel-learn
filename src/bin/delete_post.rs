extern crate diesel_learn;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_learn::*;
use std::env::args;

// cargo run --bin delete_post test
fn main() {
    use diesel_learn::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}