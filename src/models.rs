use diesel::sql_types::Bigint;
use super::schema::{posts, users};

// 用于查询
#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// 用于创建
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// 伪装成一张表，为了获取最新插入的自动自增的ID （MySQL特有）
table! {
    sequences(id) {
        id -> BigInt,
    }
}

// 用于获取id
#[derive(QueryableByName)]
// #[table_name="sequences"]
pub struct Sequence {
    #[sql_type = "Bigint"]
    pub id: i64,
}

// 用于更新
#[derive(Identifiable, AsChangeset)]
#[table_name="posts"] // 选填 当 struct 名和表名不一致时
#[primary_key(id)] // 选填 当 主键名不是 id时
pub struct PostForUpdate {
    pub id: i64,
    #[column_name = "title"]
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[table_name = "users"]
pub struct UserForm<'a> {
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
}