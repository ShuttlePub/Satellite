use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}