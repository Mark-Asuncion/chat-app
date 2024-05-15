use crate::utils::gen_uuid;

use super::DatabaseInstance;
use std::io;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    id:         String,
    email:      String,
    username:   String,
    password:   String
}

impl Account {
    pub fn new(email: &str, username: &str, password: &str) -> Self {
        Self {
            id: "".into(),
            email: email.into(),
            username: username.into(),
            password: password.into()
        }
    }

    pub async fn find_all(db: &DatabaseInstance, query: Self) -> Result<Self, sqlx::Error> {
        todo!("Account::find_all")
    }

    pub fn find(db: &DatabaseInstance, query: Self) -> Self {
        todo!("Account::find") }

    pub fn update(&self, db: &DatabaseInstance, query: Self, update: Self) -> u16 {
        todo!("Account::update")
    }

    pub fn insert(&mut self, db: &DatabaseInstance) -> io::Result<()> {
        self.id = gen_uuid(29);
        todo!("Account::save")
    }
}
