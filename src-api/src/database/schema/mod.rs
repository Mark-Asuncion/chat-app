use sqlx::postgres::PgQueryResult;

use super::DatabaseInstance;

pub mod account;
pub mod salt;

pub trait QueryExecute {
    fn insert_query(&self) -> super::query::QueryBuilder;
    async fn insert(&self, db: &DatabaseInstance) -> Result<PgQueryResult, sqlx::Error>;
}
