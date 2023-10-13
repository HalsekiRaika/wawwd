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
