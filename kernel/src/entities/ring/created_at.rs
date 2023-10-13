use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreatedAt(OffsetDateTime);

impl CreatedAt {
    pub fn new(at: impl Into<OffsetDateTime>) -> CreatedAt {
        Self(at.into())
    }
}

impl AsRef<CreatedAt> for CreatedAt {
    fn as_ref(&self) -> &CreatedAt {
        self
    }
}

impl AsRef<OffsetDateTime> for CreatedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl From<CreatedAt> for OffsetDateTime {
    fn from(value: CreatedAt) -> Self {
        value.0
    }
}