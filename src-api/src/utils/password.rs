use pbkdf2::{password_hash::{SaltString, PasswordHasher}, Pbkdf2};
use rand_core::OsRng;

use super::get_pbkdf2_params;

#[derive(Debug)]
pub struct Password {
    pub salt: String,
    pub hash: String
}

impl Password {
    pub fn hash_from(salt: &str, password: &str) -> Self {
        let password = password.as_bytes();
        let salt = SaltString::from_b64(&salt);
        if let Err(e) = salt {
            dbg!(e);
            return Self {
                salt: String::new(),
                hash: String::new()
            };
        }

        let salt = salt.unwrap();
        let password_hash = Pbkdf2.hash_password_customized(password, None, None, get_pbkdf2_params(), &salt);
        if let Err(e) = password_hash {
            dbg!(e);
            return Self {
                salt: String::new(),
                hash: String::new()
            };
        }

        let password_hash = password_hash.unwrap().to_string();


        let hashb64 = super::base64_encode(&password_hash);
        Self {
            salt: salt.to_string(),
            hash: hashb64.into()
        }
    }

    pub fn hash(password: &str) -> Self {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Pbkdf2.hash_password_customized(password, None, None, get_pbkdf2_params(), &salt);
        if let Err(e) = password_hash {
            dbg!(e);
            return Self {
                salt: String::new(),
                hash: String::new()
            };
        }
        let password_hash = password_hash.unwrap().to_string();

        let hashb64 = super::base64_encode(&password_hash);
        Self {
            salt: salt.to_string(),
            hash: hashb64
        }
    }

    pub fn verify(&self, other: &str) -> bool {
        let selfdecoded = super::base64_decode(&self.hash);
        let otherdecoded = super::base64_decode(other);
        if selfdecoded.is_empty() || otherdecoded.is_empty() {
            return false;
        }
        // dbg!(&selfdecoded, &otherdecoded);
        selfdecoded == otherdecoded
    }

    pub fn is_empty(&self) -> bool {
        self.salt.is_empty() || self.hash.is_empty()
    }
}
