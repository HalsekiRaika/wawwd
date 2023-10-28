use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct AdminToken(String);

impl AdminToken {
    pub fn new(token: impl Into<String>) -> AdminToken {
        Self(token.into())
    }
}

impl AsRef<str> for AdminToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<AdminToken> for String {
    fn from(value: AdminToken) -> Self {
        value.0
    }
}

impl Default for AdminToken {
    fn default() -> Self {
        let token = Alphanumeric
            .sample_iter(&mut rand::thread_rng())
            .take(128)
            .map(char::from)
            .collect::<String>();
        Self(token)
    }
}
