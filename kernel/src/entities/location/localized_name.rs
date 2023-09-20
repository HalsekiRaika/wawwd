use crate::entities::location::localize_id::LocalizeId;
use crate::error::KernelError;
use destructure::Destructure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct LocalizedName {
    country_code: LocalizeId,
    localize: String,
}

impl LocalizedName {
    pub fn new(
        country_code: impl Into<String>,
        localize: impl Into<String>,
    ) -> Result<LocalizedName, KernelError> {
        Ok(Self {
            country_code: LocalizeId::new(country_code)?,
            localize: localize.into(),
        })
    }

    pub fn country(&self) -> &LocalizeId {
        &self.country_code
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
