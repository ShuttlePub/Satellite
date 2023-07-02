use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(pass: impl Into<String>) -> Self {
        Self(pass.into())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}