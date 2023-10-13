use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct StartedAt(OffsetDateTime);

impl StartedAt {
    pub fn new(at: impl Into<OffsetDateTime>) -> StartedAt {
        Self(at.into())
    }
}

impl AsRef<StartedAt> for StartedAt {
    fn as_ref(&self) -> &StartedAt {
        self
    }
}

impl AsRef<OffsetDateTime> for StartedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl From<StartedAt> for OffsetDateTime {
    fn from(value: StartedAt) -> Self {
        value.0
    }
}