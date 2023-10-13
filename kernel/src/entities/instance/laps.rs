use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Laps(i32);

impl Laps {
    pub fn new(laps: impl Into<i32>) -> Laps {
        Self(laps.into())
    }
}

impl AsRef<Laps> for Laps {
    fn as_ref(&self) -> &Laps {
        self
    }
}

impl AsRef<i32> for Laps {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Laps> for i32 {
    fn from(value: Laps) -> Self {
        value.0
    }
}