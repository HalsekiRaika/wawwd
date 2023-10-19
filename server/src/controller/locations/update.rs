use crate::controller::TryIntake;
use crate::error::ServerError;
use application::transfer::UpdateLocationDto;
use geojson::feature::Id;
use geojson::{Feature, GeoJson};
use kernel::external::uuid::Uuid;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

pub struct GeoJsonToUpdateLocationDto;

impl TryIntake<GeoJson> for GeoJsonToUpdateLocationDto {
    type To = UpdateLocationDto;
    type Error = ServerError;

    //noinspection DuplicatedCode
    fn emit(&self, input: GeoJson) -> Result<Self::To, Self::Error> {
        let Feature {
            id,
            geometry,
            properties,
            ..
        } = match input {
            GeoJson::Feature(f) => Ok(f),
            _ => Err(ServerError::IO(anyhow::Error::msg(
                "Invalid format. Location-Pin registration should be \"type\": \"Feature\".",
            ))),
        }?;

        let id = id
            .map(|raw| match raw {
                Id::String(id) => Ok(id),
                _ => Err(ServerError::IO(anyhow::Error::msg(
                    "Property `id` does not allowed other than `String`",
                ))),
            })
            .transpose()?
            .map(|id| Uuid::from_str(&id))
            .transpose()
            .map_err(|e| ServerError::IO(anyhow::Error::new(e)))?
            .ok_or(ServerError::IO(anyhow::Error::msg(
                "Property `id` does not allowed empty",
            )))?;
        let geometry = geometry.ok_or(ServerError::IO(anyhow::Error::msg(
            "Property `geometry` does not allowed empty",
        )))?;
        let properties = properties.ok_or(ServerError::IO(anyhow::Error::msg(
            "Property `properties` does not allowed empty",
        )))?;

        let point = match geometry.value {
            geojson::Value::Point(point) => Ok(point),
            _ => Err(ServerError::IO(anyhow::Error::msg(
                "Invalid format. Location-Pin should be `\"type\": \"Point\".`",
            ))),
        }?;

        #[derive(Debug, Deserialize)]
        struct Extensions {
            radius: i32,
            localize: HashMap<String, String>,
        }

        let props = serde_json::from_value::<Extensions>(properties.into()).map_err(|e| {
            tracing::error!("{:?}", e);
            ServerError::IO(anyhow::Error::msg("Property cannot deserialization."))
        })?;

        Ok(UpdateLocationDto {
            id,
            latitude: point[1],
            longitude: point[0],
            radius: props.radius,
            localize: props.localize.into_iter().map(|(c, l)| (c, l)).collect(),
        })
    }
}
