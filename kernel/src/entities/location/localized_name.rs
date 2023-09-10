use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalizedName {
    country: String,
    localize: String
}

impl LocalizedName {
    pub fn new(
        country_code: impl Into<String>,
        localize: impl Into<String>
    ) -> LocalizedName {
        Self {
            country: country_code.into(),
            localize: localize.into()
        }
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn localize(&self) -> &str {
        &self.localize
    }
}

impl AsRef<LocalizedName> for LocalizedName {
    fn as_ref(&self) -> &LocalizedName {
        self
    }
}

impl From<LocalizedName> for (String, String) {
    fn from(value: LocalizedName) -> Self {
        (value.country, value.localize)
    }
}
