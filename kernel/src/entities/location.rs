mod localize;
mod localize_id;
mod localized_name;
mod location_id;

pub use self::{localize::*, localize_id::*, localized_name::*, location_id::*};

use destructure::Destructure;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::KernelError;

use super::geology::Position;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Location {
    id: LocationId,
    pos: Position,
    localize: Vec<Localize>,
}

impl Location {
    pub fn new(id: LocationId, pos: Position, localize: Vec<Localize>) -> Location {
        Self { id, pos, localize }
    }

    pub fn r#try(
        id: impl Into<Uuid>,
        pos: impl TryInto<Position, Error = KernelError>,
        localize: impl Into<Vec<Localize>>,
    ) -> Result<Location, KernelError> {
        Ok(Self {
            id: LocationId::new(id),
            pos: pos.try_into()?,
            localize: localize.into(),
        })
    }

    pub fn id(&self) -> &LocationId {
        &self.id
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn localize(&self) -> &[Localize] {
        &self.localize
    }
}

impl TryFrom<Location> for geojson::Feature {
    type Error = KernelError;
    fn try_from(value: Location) -> Result<Self, Self::Error> {
        use serde_json::{Map, Value};

        #[derive(serde::Serialize)]
        struct LocalizeExt(Map<String, Value>);

        let loc = value
            .localize
            .into_iter()
            .map(|loc| loc.into_destruct())
            .map(|des| (des.country_code.into(), des.localize.into()))
            .map(|(c, l): (String, String)| (c, Value::from(l)))
            .collect::<Vec<(String, Value)>>();

        let mut map = Map::new();
        map.extend(loc);

        let mut obj = geojson::JsonObject::new();
        obj.insert("localize".to_string(), Value::from(map));

        Ok(geojson::Feature {
            bbox: None,
            geometry: Some(value.pos.into()),
            id: Some(value.id.into()),
            properties: Some(obj),
            foreign_members: None,
        })
    }
}

impl TryFrom<geojson::GeoJson> for Location {
    type Error = KernelError;
    fn try_from(value: geojson::GeoJson) -> Result<Self, Self::Error> {
        match value {
            geojson::GeoJson::Feature(f) => {
                let geojson::Feature {
                    geometry,
                    id,
                    properties,
                    ..
                } = f;
                println!("{:#?}", properties);
                let lid = id
                    .map(|raw| -> Result<String, KernelError> {
                        let geojson::feature::Id::String(raw) = raw else {
                            return Err(KernelError::Validation {
                                msg: "`id` does not number value. This value expected `String`.",
                            });
                        };
                        Ok(raw)
                    })
                    .transpose()?
                    .map(LocationId::try_from)
                    .transpose()?
                    .ok_or(KernelError::Validation {
                        msg: "`id` does not empty value. This value must be required.",
                    })?;

                let pos = geometry.map(Position::try_from).transpose()?.ok_or(
                    KernelError::Validation {
                        msg: "`id` does not empty value. This value must be required.",
                    },
                )?;

                let props = properties
                    .map(|raw| serde_json::from_value::<Vec<Localize>>(raw.into()))
                    .transpose()
                    .map_err(|e| KernelError::TryConversion {
                        from: "serde_json::Value",
                        to: "Vec<kernel::location::LocalizedName>",
                        source: anyhow::Error::new(e),
                    })?
                    .ok_or(KernelError::Validation {
                        msg: "`localize` does not empty value. This value must be required.",
                    })?;

                let loc = Location::new(lid, pos, props);
                Ok(loc)
            }
            geojson::GeoJson::FeatureCollection(_) => Err(KernelError::UnSupportedTypeConversion {
                from: "geojson::GeoJson::FeatureCollection",
                to: "kernel::Location",
            }),
            geojson::GeoJson::Geometry(_) => Err(KernelError::UnSupportedTypeConversion {
                from: "geojson::GeoJson::Geometry",
                to: "kernel::Location",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::geology::Position;
    use crate::entities::location::{Localize, Location, LocationId};
    use geojson::Feature;

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        let loc = Location::new(
            LocationId::default(),
            Position::new(135.315684651, 64.126213518)?,
            vec![Localize::new("jp", "あいうえお")?],
        );

        let feat = Feature::try_from(loc)?;
        let geos = feat.to_string();

        println!("{:#?}", geos);
        Ok(())
    }
}
