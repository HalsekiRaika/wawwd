use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct LocationId(Uuid);

impl LocationId {
    pub fn new(id: impl Into<Uuid>) -> LocationId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for LocationId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<LocationId> for Uuid {
    fn from(id: LocationId) -> Self {
        id.0
    }
}

impl Default for LocationId {
    fn default() -> Self {
        Self::new(Uuid::new_v4())
    }
}

impl Display for LocationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "location_id: {}", self.0)
    }
}
