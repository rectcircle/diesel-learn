//! 实现分页
//! 
//! 仅仅为了演示，应该不需要，因为分页可以通过组合SQL实现
//! 参考 https://github.com/diesel-rs/diesel/blob/v1.3.0/examples/postgres/advanced-blog-cli/src/pagination.rs
use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::{QueryFragment, Query, AstPass};
use diesel::mysql::Mysql;
use diesel::sql_types::BigInt;

// 分页特质，用于创建一个Paginated特质
pub trait PaginateForQueryFragment: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

// 给所有 QueryFragment 类型添加该方法
impl<T> PaginateForQueryFragment for T 
where T: QueryFragment<Mysql>{
    /// page: 当前页数
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: 10,
            page,
            is_sub_query: true,
        }
    }
}

// https://docs.diesel.rs/diesel/query_builder/trait.QueryId.html
// QueryID 的作用是：用于实现生成的SQL语句的缓存
// 用于实现分页的结构
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    /// 子查询/表名是什么？
    query: T,
    /// 当前页码
    page: i64,
    /// 每页多行行
    per_page: i64,
    /// query 是否是子查询
    is_sub_query: bool,
}

impl<T> Paginated<T> {
    /// 每页多少条
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }

    /// 实现类似于 load(&conn) 的函数
    /// 获取 <Vec<U>, 总页数>
    pub fn load_and_count_pages<U>(self, conn: &MysqlConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<MysqlConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}

// 表示该类型可以生成完整的SQL语句，可以进行查询
impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

// 添加一系列查询方法如：execute 等
impl<T> RunQueryDsl<MysqlConnection> for Paginated<T> {}

// 核心函数：用于生成SQL
impl<T> QueryFragment<Mysql> for Paginated<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        // TODO 优化（使用ID分页）
        out.push_sql("SELECT *, COUNT(*) OVER () FROM ");
        if self.is_sub_query {
            out.push_sql("(");
        }
        self.query.walk_ast(out.reborrow())?;
        if self.is_sub_query {
            out.push_sql(")");
        }
        out.push_sql(" t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

// 以下内容为：为数据源添加分页功能

/// 包装 QuerySource 类型，是之成为 QueryFragment 类型
#[derive(Debug, Clone, Copy, QueryId)]
pub struct QuerySourceToQueryFragment<T> {
    query_source: T,
}

impl<FC, T> QueryFragment<Mysql> for QuerySourceToQueryFragment<T>
where
    FC: QueryFragment<Mysql>,
    T: QuerySource<FromClause=FC>,
{
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        self.query_source.from_clause().walk_ast(out.reborrow())?;
        Ok(())
    }
}

// 为 QuerySource 类型添加分页功能
pub trait PaginateForQuerySource: Sized {
    fn paginate(self, page: i64) -> Paginated<QuerySourceToQueryFragment<Self>>;
}

impl<T> PaginateForQuerySource for T 
where T: QuerySource {
    /// page: 当前页数
    fn paginate(self, page: i64) -> Paginated<QuerySourceToQueryFragment<Self>> {
        Paginated {
            query: QuerySourceToQueryFragment {query_source: self},
            per_page: 10,
            page,
            is_sub_query: false, // 不是子查询
        }
    }
}


