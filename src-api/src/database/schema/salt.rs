use sqlx::{Row, postgres::PgQueryResult};

use crate::database::{DatabaseUtils, DatabaseInstance};
use crate::database::query::{self};
use crate::database::query::builder;

use super::{QueryExecute, account::Account, ToQueryBuilder};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Salt {
    pub id:       String,
    pub user_id:    String,
}

impl DatabaseUtils<'_> for Salt {
    fn as_columns() -> Vec<&'static str> {
        vec!["salts.id", "salts.user_id"]
    }

    fn as_columns_alias() -> Vec<&'static str> {
        vec![
            "salts.id AS salts_id",
            "salts.user_id salts_user_id"
        ]
    }

    fn as_insert_value(&self) -> Vec<query::QueryValue> {
        vec![
            query::QueryValue::Varchar(self.id.clone()),
            query::QueryValue::Varchar(self.user_id.clone()),
        ]
    }

    fn table() -> &'static str {
        "salts"
    }

    fn pkey() -> &'static str {
        "salts.id"
    }

    fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        let salt: String = row.try_get("id").unwrap_or_default();
        let user_id: String = row.try_get("user_id").unwrap_or_default();
        Self::new(&salt, &user_id)
    }

    fn from_row_alias(row: &sqlx::postgres::PgRow) -> Self {
        let salt: String = row.try_get("salts_id").unwrap_or_default();
        let user_id: String = row.try_get("salts_user_id").unwrap_or_default();
        Self::new(&salt, &user_id)
    }
}

impl QueryExecute for Salt {
    async fn insert(&self, db: &DatabaseInstance) -> Result<PgQueryResult, sqlx::Error> {
        let mut qb = builder::QueryBuilder::new();
        qb.insert(Salt::table(), Salt::as_columns())
            .value(self.as_insert_value());
        db.execute_insert(qb).await
    }

}

impl ToQueryBuilder for Salt {
    fn insert_query(&self) -> crate::database::query::builder::QueryBuilder {
        let mut qb = builder::QueryBuilder::new();
        qb.insert(Salt::table(), Salt::as_columns())
            .value(self.as_insert_value());
        qb
    }
}

impl Salt {
    pub fn new(salt: &str, user_id: &str) -> Self {
        Self {
            id: salt.into(),
            user_id: user_id.into()
        }
    }

    pub async fn get_from(account: &Account, db: &DatabaseInstance) -> Self {
        let mut qb = builder::QueryBuilder::new();
        qb.select(Self::table(), None)
            .filter(query::filter::Filter::if_from(
                "user_id",
                "=",
                query::QueryValue::Varchar(account.id.clone()))
            );

        let res = db.fetch_one(qb).await;
        if let Err(e) = res {
            dbg!(e);
            return Self::new("","");
        }

        Self::from_row(&res.unwrap())
    }

    pub fn is_empty(&self) -> bool {
        self.id.is_empty() || self.user_id.is_empty()
    }
}
