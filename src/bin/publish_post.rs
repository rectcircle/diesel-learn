extern crate diesel_learn;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_learn::*;
use self::models::Post;
use std::env::args;

// cargo run --bin publish_post 1
fn main() {
    use diesel_learn::schema::posts::dsl::{posts, published, id};

    let id_value = args().nth(1).expect("publish_post requires a post id")
        .parse::<i64>().expect("Invalid ID");
    let connection = establish_connection();

    diesel::update(posts.filter(id.eq(id)))
        .set(published.eq(true))
        // .get_result::<Post>(&connection)
        .execute(&connection)
        .unwrap();

    let post = posts
        .find(id_value)
        .first::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id_value));
    println!("Published post {:?}", post);
}
