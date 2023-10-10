use crate::controller::form::DeleteRequest;
use crate::controller::{
    Controller, DeleteRequestToDeleteLocationDto, GeoJsonToCreateLocationDto,
    GeoJsonToUpdateLocationDto,
};
use crate::error::ServerError;
use crate::extract::GeoJson;
use crate::Handler;
use application::services::{
    CreateLocationService, DeleteLocationService, DependOnCreateLocationService,
    DependOnDeleteLocationService, DependOnUpdateLocationService, UpdateLocationService,
};
use application::transfer::{CreateLocationDto, DeleteLocationDto, UpdateLocationDto};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use geojson::{Feature, FeatureCollection};
use kernel::repository::{DependOnLocationRepository, LocationRepository};

pub async fn locations(State(handler): State<Handler>) -> Result<impl IntoResponse, ServerError> {
    let all = handler
        .location_repository()
        .find_all()
        .await?
        .into_iter()
        .map(Feature::try_from)
        .collect::<Result<Vec<Feature>, _>>()?;
    Ok(GeoJson(geojson::GeoJson::FeatureCollection(
        FeatureCollection::from_iter(all),
    )))
}

pub async fn reg_location(
    State(handler): State<Handler>,
    GeoJson(geojson): GeoJson,
) -> Result<impl IntoResponse, ServerError> {
    Controller::new(GeoJsonToCreateLocationDto, ())
        .try_intake(geojson)?
        .bypass(|input: CreateLocationDto| async {
            handler.create_location_service().create(input).await
        })
        .await?;

    Ok(())
}

pub async fn upd_location(
    State(handler): State<Handler>,
    GeoJson(geojson): GeoJson,
) -> Result<impl IntoResponse, ServerError> {
    Controller::new(GeoJsonToUpdateLocationDto, ())
        .try_intake(geojson)?
        .bypass(|input: UpdateLocationDto| async {
            handler.update_location_service().update(input).await
        })
        .await?;

    Ok(())
}

pub async fn del_location(
    State(handler): State<Handler>,
    Json(user_input): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ServerError> {
    Controller::new(DeleteRequestToDeleteLocationDto, ())
        .intake(user_input)
        .bypass(|input: DeleteLocationDto| async {
            handler.delete_location_service().delete(input).await
        })
        .await?;
    Ok(())
}
