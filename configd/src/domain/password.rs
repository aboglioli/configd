use sha2::{Digest, Sha256};
use std::fmt;

use crate::domain::Error;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: String) -> Result<Password, Error> {
        if password.is_empty() {
            return Err(Error::InvalidPassword);
        }

        Ok(Password { password })
    }

    pub fn value(&self) -> &str {
        &self.password
    }

    pub fn hash(&self) -> Result<Password, Error> {
        let mut hasher = Sha256::new();
        hasher.update(&self.password);
        let hashed_password = hex::encode(hasher.finalize());

        Password::new(hashed_password)
    }

    pub fn compare(&self, raw_password: &Password) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&raw_password.password);
        let hashed_password = hex::encode(hasher.finalize());

        self.password == hashed_password
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.password)
    }
}
