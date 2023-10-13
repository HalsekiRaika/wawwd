use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct FinishedAt(Option<OffsetDateTime>);

impl FinishedAt {
    pub fn new<T>(at: impl Into<Option<T>>) -> FinishedAt where T: Into<OffsetDateTime> {
        Self(at.into().map(Into::into))
    }

    pub fn is_finished(&self) -> bool {
        self.0.is_some()
    }
}

impl AsRef<FinishedAt> for FinishedAt {
    fn as_ref(&self) -> &FinishedAt {
        self
    }
}

impl AsRef<Option<OffsetDateTime>> for FinishedAt {
    fn as_ref(&self) -> &Option<OffsetDateTime> {
        &self.0
    }
}

impl From<FinishedAt> for Option<OffsetDateTime> {
    fn from(value: FinishedAt) -> Self {
        value.0
    }
}