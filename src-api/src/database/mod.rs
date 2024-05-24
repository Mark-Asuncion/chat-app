pub mod schema;
pub mod query;
pub mod error;

use sqlx::postgres::{Postgres, PgPoolOptions, PgQueryResult, PgRow};
use sqlx::Pool;
use std::env::var;

use self::query::{QueryBuilder, QueryValue};

pub trait AsInsertQuery {
    fn as_insert() -> String;
}

pub trait DatabaseUtils<'a>  {
    fn as_columns() -> Vec<&'a str>;
    fn as_insert_value(&self) -> Vec<QueryValue>;
    fn table() -> &'a str;
}

#[derive(Debug)]
pub struct DatabaseInstance {
    pool: Pool<Postgres>,
}

impl DatabaseInstance {
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
    let username = var("POSTGRES_USER").expect("POSTGRES_USER env not set");
    let password = var("POSTGRES_PASS").expect("POSTGRES_PASS env not set");
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
