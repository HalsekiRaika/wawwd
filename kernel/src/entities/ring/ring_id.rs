use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RingId(Uuid);

impl RingId {
    pub fn new(id: impl Into<Uuid>) -> RingId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for RingId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<RingId> for Uuid {
    fn from(value: RingId) -> Self {
        value.0
    }
}

impl Default for RingId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for RingId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
