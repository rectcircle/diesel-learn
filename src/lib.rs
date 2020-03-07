#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

// 建立连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    // 从数据库中拿到环境变量
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // 建连MySQL连接
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use diesel::r2d2;

pub fn new_connection_pool() -> r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

pub mod schema;
pub mod models;
pub mod pagination;
pub mod predicates;

use self::models::{Sequence, NewPost};

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> i64 {
    use schema::posts;

    // 构建待插入对象
    let new_post = NewPost {
        title: title,
        body: body,
    };

    // 插入到数据库
    diesel::insert_into(posts::table)
        .values(&new_post)
        // .get_result(conn) // MySQL 不支持
        .execute(conn)
        .expect("Error saving new post");

    // 获取到id
    let generated_id = diesel::sql_query("select LAST_INSERT_ID() as id")
        .load::<Sequence>(conn).expect("get_id_error").first().unwrap().id;
    generated_id
}

// 编写扩展函数一般写法

/// 声明函数
pub mod functions {
    use diesel::sql_types::{Text, BigInt, Foldable};
    sql_function!(
        /// 转换为小写
        fn lower(x: Text) -> Text;
    );
    sql_function!(
        /// 字符串长度
        fn length(x: Text) -> BigInt; // 使用BigInt的原因是字符串最大长度为4G亿
    );
    sql_function!(
        /// 字符串拼接
        #[sql_name = "concat"]
        fn concat2(x: Text, y: Text) -> BigInt; 
    );
    sql_function!(
        /// 聚合函数 sum
        #[aggregate]
        #[sql_name = "SUM"]
        fn sum<ST: Foldable>(expr: ST) -> ST::Sum;
    );
    // 无参数SQL函数
    no_arg_sql_function!(last_insert_id, BigInt, "no_arg_sql_function()");

}

pub mod helper_types {
    /// The return type of `length(expr)`
    pub type Lower<Expr> = super::functions::lower::HelperType<Expr>;
    // 其他略
}

pub mod dsl {
    pub use super::functions::*;
    pub use super::helper_types::*;
}

// cargo test
#[cfg(test)]
mod test {
    use super::*;
    use diesel::mysql::Mysql;
    use diesel::debug_query;
    use models::{PostForUpdate, Post};


    #[test]
    fn test_update() {
        use schema::posts::dsl::*;
        // 更新全部
        let query = diesel::update(posts).set(published.eq(false));
        println!("{}", debug_query::<Mysql, _>(&query));

        // 更新部分（附带where语句）
        let target = posts.filter(published.eq(true));
        let query = diesel::update(target).set(published.eq(false));
        println!("{}", debug_query::<Mysql, _>(&query));

        // 更新用户传递的对象 （需 `#[derive(Identifiable)]`）
        let post = PostForUpdate {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            published: true,
        };
        let query = diesel::update(&post).set(published.eq(false));
        println!("{}", debug_query::<Mysql, _>(&query));

        // 更新过程中自引用
        let query = diesel::update(&post).set(id.eq(id + 1));
        println!("{}", debug_query::<Mysql, _>(&query));
        
        // 更新多列（使用元组）
        let query = diesel::update(&post).set(
            (
                published.eq(false),
                title.eq("xxx")
            )
        );
        println!("{}", debug_query::<Mysql, _>(&query));

        // 更新多列（使用对象）（需实现 `#[derive(AsChangeset)]`）
        // 注意默认情况下 None 对象表示忽略更新的字段（通过[changeset_options(treat_none_as_null="true")] 更改）
        let query = diesel::update(&post).set(&post);
        println!("{}", debug_query::<Mysql, _>(&query));

        let connection = establish_connection();

        // 执行查询
        let query = diesel::update(&post).set(&post);
        let effect_lines = query.execute(&connection).unwrap();
        println!("{}", effect_lines);
        // let r = query.get_result::<Post>(&connection); // mysql 不支持
        // posts.save_changes(&connection); // mysql 不支持
    }

    #[test]
    fn test_insert() {
        use schema::users::dsl::*;
        use models::UserForm;

        // 如果全有默认值，可以以如下语法插入
        let query = diesel::insert_into(users).default_values();
        println!("{}", debug_query::<Mysql, _>(&query));

        // 插入指定值单个
        let query = diesel::insert_into(users)
            .values(name.eq("Sean"));
        println!("{}", debug_query::<Mysql, _>(&query));

        // 插入指定值多个
        let query = diesel::insert_into(users)
            .values((name.eq("Tess"), hair_color.eq("Brown")));
        println!("{}", debug_query::<Mysql, _>(&query));
        
        // 插入对象
        let user_form = UserForm {
            name: "Sean",
            hair_color: Some("Black"), // 如果是 None 将使用默认值
        };
        let query = diesel::insert_into(users)
            .values(&user_form);
        println!("{}", debug_query::<Mysql, _>(&query));
        
        // 批量插入1
        let v= vec![name.eq("Sean"), name.eq("Tess")];
        let query = diesel::insert_into(users)
            .values(&v);
        println!("{}", debug_query::<Mysql, _>(&query));

        // 批量插入2
        let v= vec![Some(name.eq("Sean")), None];
        let query = diesel::insert_into(users)
            .values(&v);
        println!("{}", debug_query::<Mysql, _>(&query));
        
        // 批量插入3
        let v= vec![
                (name.eq("Sean"), hair_color.eq("Black")),
                (name.eq("Tess"), hair_color.eq("Brown")),
            ];
        let query = diesel::insert_into(users)
            .values(&v);
        println!("{}", debug_query::<Mysql, _>(&query));
        
        // 批量插入4
        let v= vec![
                (name.eq("Sean"), Some(hair_color.eq("Black"))),
                (name.eq("Ruby"), None),
            ];
        let query = diesel::insert_into(users)
            .values(&v);
        println!("{}", debug_query::<Mysql, _>(&query));

        // 批量插入5
        let v= vec![
                user_form,
                UserForm {
                    name: "Sean",
                    hair_color: Some("Black"),
                }
            ];
        let query = diesel::insert_into(users)
            .values(&v);
        println!("{}", debug_query::<Mysql, _>(&query));
    }

    #[test]
    fn test_extend_query_builder() {
        use schema::users::dsl::*;
        use super::dsl::*;

        let conn = establish_connection();

        {
            // 自定义函数
            let query = users.select(lower(name));
            println!("{}", debug_query::<Mysql, _>(&query));

            let query = users.select(concat2(name, "name"));
            // 等价于如下操作符的方式
            // let query = users.select(name.concat("name"));
            println!("{}", debug_query::<Mysql, _>(&query));
            
            let query = diesel::select(last_insert_id);
            let result = query.get_result::<i64>(&conn).unwrap();
            println!("sql = {}, result = {}", debug_query::<Mysql, _>(&query), result);

        }

        // 自定义SQL扩展查询DSL
        {
            use schema::posts::dsl::*;
            use super::pagination::{PaginateForQueryFragment, PaginateForQuerySource};

            let query = posts.filter(id.ge(0).and(id.ge(0))).paginate(1).per_page(3);
            println!("{:?}", id);
            let r = query.load_and_count_pages::<Post>(&conn).unwrap();
            println!("sql = {}, {:?}", debug_query::<Mysql, _>(&query), r);
            
            let query = posts.paginate(1).per_page(3);
            // let query = posts.select(id);
            println!("sql = {}", debug_query::<Mysql, _>(&query));
        }

        // 自定义运算符
        {
            use predicates::*;
            use schema::posts::dsl::*;
            id.bit_and(1);
            let query = posts.filter(id.ge(id.bit_and(1)));
            println!("sql = {}", debug_query::<Mysql, _>(&query));
        }
    }

    #[test]
    fn test_conn_pool(){
        use schema::posts;
        let pool = new_connection_pool();
        let conn = pool.get().unwrap();

        let posts = posts::table.load::<Post>(&conn);

        println!("{:?}", posts);
    }
}
