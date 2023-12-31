use std::str::FromStr;
use crate::controller::form::DeleteRequest;
use crate::controller::{
    Controller, DeleteRequestToDeleteLocationDto, GeoJsonToCreateLocationDto,
    GeoJsonToUpdateLocationDto,
};
use crate::error::ServerError;
use crate::extract::GeoJson;
use crate::AppHandler;
use application::services::{
    CreateLocationService, DeleteLocationService, DependOnCreateLocationService,
    DependOnDeleteLocationService, DependOnUpdateLocationService, UpdateLocationService,
};
use application::transfer::{CreateLocationDto, DeleteLocationDto, UpdateLocationDto};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum::headers::{ETag, HeaderMapExt, IfNoneMatch};
use geojson::{Feature, FeatureCollection};
use kernel::repository::{DependOnLocationRepository, LocationRepository};
use kernel::volatiles::{DependOnLocationETagCache, LocationETagCache};

use inner::ResType;

pub async fn locations(
    State(handler): State<AppHandler>,
    header: HeaderMap,
) -> Result<impl IntoResponse, ServerError> {
    if let Some(etag) = header.typed_try_get::<IfNoneMatch>()? {
        if let Some(cache) = handler.location_e_tag_cache().find().await? {
            println!("{:?}", etag);
            let cached = &ETag::from_str(cache.as_ref())
                .map_err(|e| {
                    tracing::error!("ETag parse error: {:?}", e.to_string());
                    ServerError::IO(anyhow::Error::new(e))
                })?;
            println!("{:?}", cached);
            if !etag.precondition_passes(cached) {
                return Ok(ResType::NotModified(StatusCode::NOT_MODIFIED));
            }
        }
    }

    let all = handler
        .location_repository()
        .find_all()
        .await?
        .into_iter()
        .map(Feature::try_from)
        .collect::<Result<Vec<Feature>, _>>()?;
    let find: Option<String> = handler.location_e_tag_cache()
        .find()
        .await?
        .map(Into::into);

    Ok(ResType::Ok(
        StatusCode::OK,
        find,
        GeoJson(geojson::GeoJson::FeatureCollection(FeatureCollection::from_iter(all)))
    ))
}

pub async fn reg_location(
    State(handler): State<AppHandler>,
    GeoJson(geojson): GeoJson,
) -> Result<impl IntoResponse, ServerError> {
    Controller::new(GeoJsonToCreateLocationDto, ())
        .try_intake(geojson)?
        .bypass(|input: CreateLocationDto| async {
            handler.create_location_service().create(input).await
        })
        .await?;

    Ok(StatusCode::CREATED)
}

pub async fn upd_location(
    State(handler): State<AppHandler>,
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
    State(handler): State<AppHandler>,
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

mod inner {
    use axum::headers::HeaderValue;
    use axum::http::{HeaderMap, StatusCode};
    use axum::response::{IntoResponse, Response};
    use crate::extract::GeoJson;

    pub(super) enum ResType {
        NotModified(StatusCode),
        Ok(StatusCode, Option<String>, GeoJson),
    }

    impl IntoResponse for ResType {
        fn into_response(self) -> Response {
            match self {
                ResType::NotModified(status) => status.into_response(),
                ResType::Ok(status, etag, geojson) => {
                    let mut headers = HeaderMap::new();
                    if let Some(etag) = etag {
                        headers.insert("ETag", HeaderValue::from_str(&etag).unwrap());
                    }
                    (status, headers, geojson).into_response()
                },
            }
        }
    }
}
