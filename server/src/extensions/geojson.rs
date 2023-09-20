use axum::response::{IntoResponse, Response};

pub struct GeoJson<T>(T);

impl<T> IntoResponse for GeoJson<T> {
    fn into_response(self) -> Response {
        todo!()
    }
}