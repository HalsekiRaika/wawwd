use serde::{Deserialize, Serialize};
use crate::error::KernelError;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(latitude: impl Into<f64>) -> Result<Latitude, KernelError> {
        let lat = latitude.into();
        if !(-90f64..=90f64).contains(&lat) {
            return Err(KernelError::Validation {
                msg: "latitude values are invalid. latitude should takes -90~90 degrees."
            })
        }
        Ok(Self(lat))
    }
}

impl From<Latitude> for f64 {
    fn from(value: Latitude) -> Self {
        value.0
    }
}

impl AsRef<f64> for Latitude {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_test() -> anyhow::Result<()> {
        let lat = Latitude::new(-100.1354843213f64);
        assert!(lat.is_err());
        let lat = Latitude::new(60.651654851654f64);
        assert!(lat.is_ok());
        let lat = Latitude::new(100.51654846513f64);
        assert!(lat.is_err());
        Ok(())
    }
}