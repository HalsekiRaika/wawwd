use crate::controller::{Controller, CreateImageRequest, CreateImageRequestToCreateImageDto};
use crate::error::ServerError;
use crate::AppHandler;
use application::services::{DependOnExportImageService, ExportImageService};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn reg_images(
    State(handler): State<AppHandler>,
    Json(request): Json<CreateImageRequest>,
) -> Result<impl IntoResponse, ServerError> {
    Controller::new(CreateImageRequestToCreateImageDto, ())
        .intake(request)
        .bypass(|input| async { handler.export_image_service().export(input).await })
        .await?;
    Ok(StatusCode::CREATED)
}
