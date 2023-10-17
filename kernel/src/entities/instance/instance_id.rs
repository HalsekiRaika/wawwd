use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InstanceId(Uuid);

impl InstanceId {
    pub fn new(id: impl Into<Uuid>) -> InstanceId {
        Self(id.into())
    }
}

impl AsRef<InstanceId> for InstanceId {
    fn as_ref(&self) -> &InstanceId {
        self
    }
}

impl AsRef<Uuid> for InstanceId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<InstanceId> for Uuid {
    fn from(value: InstanceId) -> Self {
        value.0
    }
}

impl Default for InstanceId {
    fn default() -> Self {
        Self::new(Uuid::new_v4())
    }
}

impl Display for InstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}