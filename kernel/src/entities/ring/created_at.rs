use crate::error::KernelError;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreatedAt(#[serde(with = "time::serde::iso8601")] OffsetDateTime);

impl CreatedAt {
    pub fn new(at: impl Into<OffsetDateTime>) -> CreatedAt {
        Self(at.into())
    }

    pub fn format(self) -> Result<String, KernelError> {
        self.0
            .format(&Iso8601::DEFAULT)
            .map_err(|e| KernelError::TryConversion {
                from: "kernel::entities::ring::CreatedAt",
                to: "String",
                source: anyhow::Error::new(e),
            })
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

impl Default for CreatedAt {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}
