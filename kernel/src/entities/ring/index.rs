use serde::{Deserialize, Serialize};
use crate::error::KernelError;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Index(i32);

impl Index {
    pub fn new(index: impl Into<i32>) -> Result<Index, KernelError> {
        let index = index.into();
        if index < 0 && index > 69 { 
            return Err(KernelError::Validation {
                msg: "index value should be 0~69",
            })
        }
        Ok(Self(index))
    }
}

impl From<Index> for i32 {
    fn from(value: Index) -> Self {
        value.0
    }
}

impl AsRef<i32> for Index {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}