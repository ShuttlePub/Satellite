use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}