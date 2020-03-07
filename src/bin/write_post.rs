extern crate diesel_learn;
extern crate diesel;

use self::diesel_learn::*;
use std::io::{stdin, Read};

//cargo run --bin write_post
fn main() {
    // 创建连接
    let connection = establish_connection();

    // 获取用户输入
    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    // 执行插入
    let id = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";