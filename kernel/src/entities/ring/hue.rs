use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct HueColor(i32);

impl HueColor {
    pub fn new(h: impl Into<i32>) -> HueColor {
        let h = h.into();
        let h = if h >= 360 { h % 360 } else { h };
        Self(h)
    }
}

impl AsRef<HueColor> for HueColor {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<i32> for HueColor {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<HueColor> for i32 {
    fn from(value: HueColor) -> Self {
        value.0
    }
}