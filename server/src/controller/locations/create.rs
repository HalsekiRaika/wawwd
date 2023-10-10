use crate::controller::TryIntake;
use crate::error::ServerError;
use application::transfer::CreateLocationDto;
use geojson::{Feature, GeoJson};
use serde::Deserialize;
use std::collections::HashMap;

pub struct GeoJsonToCreateLocationDto;

impl TryIntake<GeoJson> for GeoJsonToCreateLocationDto {
    type To = CreateLocationDto;
    type Error = ServerError;

    // noinspection DuplicatedCode
    fn emit(&self, input: GeoJson) -> Result<Self::To, Self::Error> {
        let Feature {
            geometry,
            properties,
            ..
        } = match input {
            GeoJson::Feature(f) => Ok(f),
            _ => Err(ServerError::IO(anyhow::Error::msg(
                "Invalid format. Location-Pin registration should be \"type\": \"Feature\".",
            ))),
        }?;

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
            localize: HashMap<String, String>,
        }

        let props = serde_json::from_value::<Extensions>(properties.into()).map_err(|e| {
            tracing::error!("{:?}", e);
            ServerError::IO(anyhow::Error::msg("Property cannot deserialization."))
        })?;

        Ok(CreateLocationDto {
            latitude: point[1],
            longitude: point[0],
            localize: props
                .localize
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect::<Vec<_>>(),
        })
    }
}
