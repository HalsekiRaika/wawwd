use crate::error::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocalizeId(String);

impl LocalizeId {
    pub fn new(id: impl Into<String>) -> Result<LocalizeId, KernelError> {
        let id = id.into();

        if id.len() > 4 {
            return Err(KernelError::Validation {
                msg: "LocalizeId should be less than 4 char.",
            });
        }

        Ok(Self(id))
    }
    
    pub fn unchecked_new(id: impl Into<String>) -> LocalizeId {
        Self(id.into())
    }
}

impl AsRef<str> for LocalizeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<LocalizeId> for String {
    fn from(value: LocalizeId) -> Self {
        value.0
    }
}
