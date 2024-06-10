use sqlx::Row;
use sqlx::postgres::PgQueryResult;

use crate::database::DatabaseInstance;
use crate::database::query::QueryBuilder;
use crate::error;
use crate::utils::gen_uuid;

use super::super::DatabaseUtils;
use super::super::query;
use super::QueryExecute;
// use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    pub id:         String,
    pub email:      String,
    pub username:   String,
    pub password:   String
}

impl DatabaseUtils<'_> for Account {
    fn as_columns() -> Vec<&'static str> {
        vec!["id", "email", "username", "password"]
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

    fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        let id: String = row.try_get("id").unwrap_or_default();
        let email: String = row.try_get("email").unwrap_or_default();
        let username: String = row.try_get("username").unwrap_or_default();
        let password: String = row.try_get("password").unwrap_or_default();
        Self::from(&id, &email, &username, &password)
    }
}

impl QueryExecute for Account {
    async fn insert(&self, db: &DatabaseInstance) -> Result<PgQueryResult, sqlx::Error> {
        let mut qb = query::QueryBuilder::new();

        qb.insert(Account::table(), Account::as_columns())
            .value(self.as_insert_value());
        db.execute_insert(qb).await
    }

    fn insert_query(&self) -> crate::database::query::QueryBuilder {
        let mut qb = query::QueryBuilder::new();
        qb.insert(Account::table(), Account::as_columns())
            .value(self.as_insert_value());
        qb
    }
}

impl Account {
    // pub fn from(json: Value) -> Result<Self, error::Error> {
    //     let email    =  &json["email"];
    //     let username =  &json["username"];
    //     let password =  &json["password"];
    //     if email == &Value::Null && username == &Value::Null
    //         || password == &Value::Null {
    //         return Err(error::Error::new(ErrTypes::MissingFields, "Email or Username"));
    //     }
    //     let email = email.as_str().unwrap_or_default().to_string();
    //     let username = username.as_str().unwrap_or_default().to_string();
    //     let password = password.as_str().unwrap_or_default().to_string();
    //     Ok(Self {
    //         id: "".into(),
    //         email,
    //         username,
    //         password
    //     })
    // }

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
