use crate::entities::location::LocalizeId;
use crate::entities::location::LocalizeName;
use crate::error::KernelError;
use destructure::Destructure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Destructure)]
pub struct Localize {
    country_code: LocalizeId,
    localize: LocalizeName,
}

impl Localize {
    pub fn new(
        country_code: impl Into<String>,
        localize: impl Into<String>,
    ) -> Result<Localize, KernelError> {
        Ok(Self {
            country_code: LocalizeId::new(country_code)?,
            localize: LocalizeName::new(localize),
        })
    }

    pub fn country(&self) -> &LocalizeId {
        &self.country_code
    }

    pub fn localize(&self) -> &LocalizeName {
        &self.localize
    }
}

impl AsRef<Localize> for Localize {
    fn as_ref(&self) -> &Localize {
        self
    }
}
