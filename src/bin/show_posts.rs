extern crate diesel_learn;
extern crate diesel;

use self::diesel_learn::*;
use self::models::*;
use self::diesel::prelude::*;

// cargo run --bin show_posts
fn main() {
    // 导入代码
    use diesel_learn::schema::posts::dsl::*;

    // 创建连接
    let connection = establish_connection();
    // 查询
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    // 打印
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post);
    }
}