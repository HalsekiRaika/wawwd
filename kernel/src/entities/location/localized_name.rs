use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizeName(String);

impl LocalizeName {
    pub fn new(localize: impl Into<String>) -> LocalizeName {
        Self(localize.into())
    }
}

impl AsRef<str> for LocalizeName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<LocalizeName> for String {
    fn from(value: LocalizeName) -> Self {
        value.0
    }
}
