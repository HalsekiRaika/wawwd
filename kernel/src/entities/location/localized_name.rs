use serde::{Deserialize, Serialize};
use crate::entities::image::AsTraitType;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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

impl AsTraitType for LocalizeName {
    fn as_trait_type(&self) -> &str {
        "location"
    }
}