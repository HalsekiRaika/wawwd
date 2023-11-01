use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Radius(i32);

impl Radius {
    pub fn new(rad: impl Into<i32>) -> Radius {
        Self(rad.into())
    }
}

impl AsRef<i32> for Radius {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Radius> for i32 {
    fn from(value: Radius) -> Self {
        value.0
    }
}
