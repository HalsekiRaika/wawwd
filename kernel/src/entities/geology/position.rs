use destructure::Destructure;
use serde::{Deserialize, Serialize};
use crate::error::KernelError;

use super::{Latitude, Longitude};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Destructure)]
pub struct Position {
    x: Longitude,
    y: Latitude,
}

impl Position {
    pub fn new(longitude: impl Into<f64>, latitude: impl Into<f64>) -> Result<Position, KernelError> {
        Ok(Self { x: Longitude::new(longitude)?, y: Latitude::new(latitude)?, })
    }

    pub fn x(&self) -> &Longitude {
        &self.x
    }

    pub fn y(&self) -> &Latitude {
        &self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() -> anyhow::Result<()> {
        let pos = Position::new(130.461184534f64, 95.54124248f64);
        assert!(pos.is_err());
        let pos = Position::new(180.156845681f64, 86.15641254f64);
        assert!(pos.is_err());
        let pos = Position::new(130.126548651f64, 45.15684563f64);
        assert!(pos.is_ok());
        Ok(())
    }
}