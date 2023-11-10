use serde::{Deserialize, Serialize};
use crate::entities::image::AsTraitType;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct HueColor(i32);

impl HueColor {
    pub fn new(h: impl Into<i32>) -> HueColor {
        let h = h.into();
        let h = if h >= 360 { h % 360 } else { h };
        Self(h)
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

impl AsTraitType for HueColor {
    fn as_trait_type(&self) -> &str {
        "color"
    }
}