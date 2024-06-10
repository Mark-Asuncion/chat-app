pub mod schema;
pub mod query;

use sqlx::postgres::{Postgres, PgPoolOptions, PgQueryResult, PgRow};
use sqlx::Pool;
use std::env::var;

use self::query::{builder::QueryBuilder, QueryValue};

pub trait DatabaseUtils<'a>  {
    fn as_columns() -> Vec<&'a str>;
    fn as_columns_alias() -> Vec<&'a str>;
    fn as_insert_value(&self) -> Vec<QueryValue>;
    fn table() -> &'a str;
    fn pkey() -> &'a str;
    fn from_row(row: &PgRow) -> Self;
    fn from_row_alias(row: &PgRow) -> Self;
}

#[derive(Debug)]
pub struct DatabaseInstance {
    pool: Pool<Postgres>,
}

impl DatabaseInstance {
    pub async fn execute_insert_mult(&self, mut vq: Vec<QueryBuilder>) -> Result<PgQueryResult, sqlx::Error> {
        let mut res = String::new();
        for q in vq.iter_mut() {
            res += &q.build();
        }
        dbg!(&res);
        Ok(sqlx::raw_sql(&res)
            .execute(&self.pool)
            .await?)
    }

    pub async fn execute_insert(&self, mut q: QueryBuilder) -> Result<PgQueryResult, sqlx::Error> {
        let q = q.build();
        dbg!(&q);
        Ok(sqlx::query(&q)
            .execute(&self.pool)
            .await?)
    }

    pub async fn fetch_one(&self, mut q: QueryBuilder) -> Result<PgRow, sqlx::Error> {
        let q = q.build();
        dbg!(&q);
        Ok(sqlx::query(&q)
            .fetch_one(&self.pool).await?)
    }
}

pub async fn init() -> Result<DatabaseInstance, sqlx::Error> {
    let username = var("CHA_POSTGRES_USER").expect("CHA_POSTGRES_USER env not set");
    let password = var("CHA_POSTGRES_PASS").expect("CHA_POSTGRES_PASS env not set");
    let connection = format!("postgres://{}:{}@localhost:5432/ChatApp", username, password);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection.as_str())
        .await?;
    // dbg!(&pool);
    Ok(DatabaseInstance {
        pool
    })
}
