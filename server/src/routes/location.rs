use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use kernel::repository::{DependOnLocationRepository, LocationRepository};
use crate::error::ServerError;
use crate::Handler;

pub async fn locations(
    State(handler): State<Handler>
) -> Result<impl IntoResponse, ServerError> {
    let all = handler.location_repository()
        .find_all()
        .await?;
    Ok(Json(all))
}