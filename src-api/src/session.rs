use actix_session::{Session, SessionInsertError};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MSession {
    pub user_id:     String,
    pub authorized:  bool
}

impl MSession {
    pub fn from(session: &Session) -> Self {
        let authorized = session.get::<bool>("authorized").unwrap_or_default().unwrap_or_default();
        let user_id = session.get::<String>("user_id").unwrap_or_default().unwrap_or_default();
        Self {
            authorized,
            user_id
        }
    }

    pub fn insert(&self, session: &Session) -> Result<(), SessionInsertError> {
        session.insert("user_id", self.user_id.clone())?;
        session.insert("authorized", self.authorized.clone())?;
        Ok(())
    }
}
