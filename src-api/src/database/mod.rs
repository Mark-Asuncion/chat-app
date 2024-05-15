pub mod schema;
pub mod query;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Postgres, Pool};
use std::env::var;

pub trait AsInsertQuery {
    fn as_insert() -> String;
}

#[derive(Debug)]
pub struct DatabaseInstance {
    pool: Pool<Postgres>,
}

impl DatabaseInstance {
}

pub async fn init() -> Result<DatabaseInstance, Error> {
    let username = var("POSTGRES_USER").expect("POSTGRES_USER env not set");
    let password = var("POSTGRES_PASS").expect("POSTGRES_PASS env not set");
    let connection = format!("postgres://{}:{}@localhost:5432/ChatApp", username, password);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection.as_str())
        .await?;
    dbg!(&pool);
    Ok(DatabaseInstance {
        pool
    })
}
