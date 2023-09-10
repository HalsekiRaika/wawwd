use serde::{Deserialize, Serialize};
use crate::error::KernelError;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(longitude: impl Into<f64>) -> Result<Longitude, KernelError> {
        let lon = longitude.into();
        if !(-180f64..=180f64).contains(&lon) {
            return Err(KernelError::Validation {
                msg: "longitude values are invalid. longitude should takes -90~90 degrees."
            })
        }
        Ok(Self(lon))
    }
}

impl From<Longitude> for f64 {
    fn from(value: Longitude) -> Self {
        value.0
    }
}

impl AsRef<f64> for Longitude {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}