use sqlx::Row;
use sqlx::postgres::PgQueryResult;

use crate::database::DatabaseInstance;
use crate::database::query::builder;
use crate::utils::gen_uuid;

use super::super::DatabaseUtils;
use super::super::query;
use super::QueryExecute;
use super::ToQueryBuilder;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    pub id:         String,
    pub email:      String,
    pub username:   String,
    pub password:   String
}

impl DatabaseUtils<'_> for Account {
    fn as_columns() -> Vec<&'static str> {
        vec!["accounts.id", "accounts.email", "accounts.username", "accounts.password"]
    }

    fn as_columns_alias() -> Vec<&'static str> {
        vec![
            "accounts.id AS accounts_id",
            "accounts.email AS accounts_email",
            "accounts.username AS accounts_username",
            "accounts.password AS accounts_password"
        ]
    }

    fn as_insert_value(&self) -> Vec<query::QueryValue> {
        vec![
            query::QueryValue::Varchar(self.id.clone()),
            query::QueryValue::Varchar(self.email.clone()),
            query::QueryValue::Varchar(self.username.clone()),
            query::QueryValue::Varchar(self.password.clone())
        ]
    }

    fn table() -> &'static str {
        "accounts"
    }

    fn pkey() -> &'static str {
        "accounts.id"
    }

    fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        let id: String = row.try_get("id").unwrap_or_default();
        let email: String = row.try_get("email").unwrap_or_default();
        let username: String = row.try_get("username").unwrap_or_default();
        let password: String = row.try_get("password").unwrap_or_default();
        Self::from(&id, &email, &username, &password)
    }

    fn from_row_alias(row: &sqlx::postgres::PgRow) -> Self {
        let id: String = row.try_get("accounts_id").unwrap_or_default();
        let email: String = row.try_get("accounts_email").unwrap_or_default();
        let username: String = row.try_get("accounts_username").unwrap_or_default();
        let password: String = row.try_get("accounts_password").unwrap_or_default();
        Self::from(&id, &email, &username, &password)
    }
}

impl QueryExecute for Account {
    async fn insert(&self, db: &DatabaseInstance) -> Result<PgQueryResult, sqlx::Error> {
        let mut qb = builder::QueryBuilder::new();

        qb.insert(Account::table(), Account::as_columns())
            .value(self.as_insert_value());
        db.execute_insert(qb).await
    }
}

impl ToQueryBuilder for Account {
    fn insert_query(&self) -> crate::database::query::builder::QueryBuilder {
        let mut qb = builder::QueryBuilder::new();
        qb.insert(Account::table(), Account::as_columns())
            .value(self.as_insert_value());
        qb
    }
}

impl Account {
    pub fn gen_uuid(&mut self) {
        self.id = gen_uuid();
    }

    pub fn from(id: &str, email: &str, username: &str, password: &str) -> Self {
        Self {
            id: id.into(),
            email: email.into(),
            username: username.into(),
            password: password.into()
        }
    }

    pub fn new(email: &str, username: &str, password: &str) -> Self {
        let id = gen_uuid();
        Self {
            id,
            email: email.into(),
            username: username.into(),
            password: password.into()
        }
    }

    pub fn to_read_only(&self) -> ReadOnlyAccount {
        ReadOnlyAccount { 
            id: self.id.clone(),
            email: self.email.clone(),
            username: self.username.clone()
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReadOnlyAccount {
    pub id:         String,
    pub email:      String,
    pub username:   String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginRegisterInfo {
    email:    Option<String>,
    username: Option<String>,
    password: String
}

impl LoginRegisterInfo {
    pub fn to_account(&self) -> Account {
        Account::new(&self.email.as_ref().unwrap_or(&"".to_string()),
            &self.username.as_ref().unwrap_or(&"".to_string()),
            &self.password)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ValidateForms {
    pub email:    Option<String>,
    pub username: Option<String>,
}
