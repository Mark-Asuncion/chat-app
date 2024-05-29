use crate::error;
use crate::error::ErrTypes;
use crate::utils::gen_uuid;

use super::super::DatabaseUtils;
use super::super::query;
use serde_json::Value;

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
}

impl Account {
    pub fn from(json: Value) -> Result<Self, error::Error> {
        let email    =  &json["email"];
        let username =  &json["username"];
        let password =  &json["password"];
        if email == &Value::Null && username == &Value::Null
            || password == &Value::Null {
            return Err(error::Error::new(ErrTypes::MissingFields, "Email or Username"));
        }
        let email = email.as_str().unwrap_or_default().to_string();
        let username = username.as_str().unwrap_or_default().to_string();
        let password = password.as_str().unwrap_or_default().to_string();
        Ok(Self {
            id: "".into(),
            email,
            username,
            password
        })
    }

    pub fn gen_uuid(&mut self) {
        self.id = gen_uuid();
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
