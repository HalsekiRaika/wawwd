use crate::error::KernelError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

impl From<LocationId> for String {
    fn from(value: LocationId) -> Self {
        value.0.to_string()
    }
}

impl From<LocationId> for geojson::feature::Id {
    fn from(value: LocationId) -> Self {
        Self::String(value.into())
    }
}

impl TryFrom<String> for LocationId {
    type Error = KernelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LocationId::new(Uuid::from_str(value.as_str()).map_err(
            |e| KernelError::TryConversion {
                from: "&str",
                to: "Uuid",
                source: anyhow::Error::new(e),
            },
        )?))
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
