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
        let hashed_password =
            bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).map_err(Error::PasswordHash)?;

        Password::new(hashed_password)
    }

    pub fn compare(&self, raw_password: &Password) -> bool {
        bcrypt::verify(raw_password.value(), &self.password).unwrap_or(false)
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.password)
    }
}
