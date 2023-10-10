use crate::error::KernelError;
use destructure::Destructure;
use serde::{Deserialize, Serialize};

use super::{Latitude, Longitude};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Destructure)]
pub struct Position {
    x: Longitude,
    y: Latitude,
}

impl Position {
    pub fn new(
        longitude: impl Into<f64>,
        latitude: impl Into<f64>,
    ) -> Result<Position, KernelError> {
        Ok(Self {
            x: Longitude::new(longitude)?,
            y: Latitude::new(latitude)?,
        })
    }

    pub fn x(&self) -> &Longitude {
        &self.x
    }

    pub fn y(&self) -> &Latitude {
        &self.y
    }
}

impl From<Position> for geo_types::Geometry {
    fn from(value: Position) -> Self {
        Self::Point(geo_types::Point::new(value.x.into(), value.y.into()))
    }
}

impl From<Position> for geo_types::Point {
    fn from(value: Position) -> Self {
        geo_types::Point::new(value.x.into(), value.y.into())
    }
}

impl TryFrom<geo_types::Geometry> for Position {
    type Error = KernelError;

    fn try_from(value: geo_types::Geometry) -> Result<Self, Self::Error> {
        match value {
            geo_types::Geometry::Point(point) => {
                let (x, y) = point.x_y();
                Ok(Self::new(x, y)?)
            }
            _ => Err(KernelError::UnSupportedTypeConversion {
                from: "with the exception of geo_types::Geometry::Point",
                to: "kernel::entities::geometry::Position",
            }),
        }
    }
}

impl From<Position> for geojson::Geometry {
    fn from(value: Position) -> Self {
        geojson::Geometry::from(geojson::Value::from(&geo_types::Point::from(value)))
    }
}

impl TryFrom<geojson::Geometry> for Position {
    type Error = KernelError;
    fn try_from(geometry: geojson::Geometry) -> Result<Self, Self::Error> {
        use geojson::Value;

        match geometry.value {
            Value::Point(point) => Ok(Position::new(point[0], point[1])?),
            _ => Err(KernelError::UnSupportedTypeConversion {
                from: "with exception of geojson::Value::Point",
                to: "kernel::geology::Position",
            }),
        }
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
