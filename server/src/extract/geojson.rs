use axum::body::{Bytes, HttpBody};
use axum::extract::rejection::BytesRejection;
use axum::extract::FromRequest;
use axum::headers::HeaderMap;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{async_trait, BoxError};
use bytes::{BufMut, BytesMut};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct GeoJson(pub geojson::GeoJson);

#[async_trait]
impl<S, B> FromRequest<S, B> for GeoJson
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = GeoJsonRejection;
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        if geo_json_content_type(req.headers()) {
            let bytes = Bytes::from_request(req, state).await?;
            let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            let value = match geojson::GeoJson::deserialize(deserializer) {
                Ok(geojson) => geojson,
                Err(e) => {
                    let reject = match e.classify() {
                        serde_json::error::Category::Data => JsonDataError::from(e).into(),
                        serde_json::error::Category::Syntax
                        | serde_json::error::Category::Eof
                        | serde_json::error::Category::Io => JsonSyntaxError::from(e).into(),
                    };
                    return Err(reject);
                }
            };
            Ok(GeoJson(value))
        } else {
            Err(MissingGeoJsonContentType.into())
        }
    }
}

fn geo_json_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(axum::http::header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_geo_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "geo" || mime.suffix().map_or(false, |name| name == "json"));

    is_geo_json_content_type
}

impl IntoResponse for GeoJson {
    fn into_response(self) -> Response {
        let mut writer = BytesMut::with_capacity(128).writer();
        match serde_json::to_writer(&mut writer, &self.0) {
            Ok(_) => (
                [(axum::http::header::CONTENT_TYPE, "application/geo+json")],
                writer.into_inner().freeze(),
            )
                .into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                [(
                    axum::http::header::CONTENT_TYPE,
                    mime::TEXT_PLAIN_UTF_8.as_ref(),
                )],
                e.to_string(),
            )
                .into_response(),
        }
    }
}

pub enum GeoJsonRejection {
    ByteRejection(BytesRejection),
    JsonDataError(JsonDataError),
    JsonSyntaxError(JsonSyntaxError),
    MissingGeoJsonContentType(MissingGeoJsonContentType),
    InternalError(anyhow::Error),
}

impl IntoResponse for GeoJsonRejection {
    fn into_response(self) -> Response {
        match self {
            GeoJsonRejection::ByteRejection(e) => e.into_response(),
            GeoJsonRejection::JsonDataError(e) => {
                (StatusCode::BAD_REQUEST, e.0.to_string()).into_response()
            }
            GeoJsonRejection::JsonSyntaxError(e) => {
                (StatusCode::BAD_REQUEST, e.0.to_string()).into_response()
            }
            GeoJsonRejection::MissingGeoJsonContentType(_) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Expected request with `Content-Type: application/geo+json`".to_string(),
            )
                .into_response(),
            GeoJsonRejection::InternalError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

pub struct MissingGeoJsonContentType;

impl From<MissingGeoJsonContentType> for GeoJsonRejection {
    fn from(value: MissingGeoJsonContentType) -> Self {
        Self::MissingGeoJsonContentType(value)
    }
}

impl From<BytesRejection> for GeoJsonRejection {
    fn from(value: BytesRejection) -> Self {
        Self::ByteRejection(value)
    }
}

pub struct JsonDataError(serde_json::error::Error);

impl From<serde_json::error::Error> for JsonDataError {
    fn from(value: serde_json::error::Error) -> Self {
        Self(value)
    }
}

impl From<JsonDataError> for GeoJsonRejection {
    fn from(value: JsonDataError) -> Self {
        Self::JsonDataError(value)
    }
}

pub struct JsonSyntaxError(serde_json::error::Error);

impl From<serde_json::error::Error> for JsonSyntaxError {
    fn from(value: serde_json::error::Error) -> Self {
        Self(value)
    }
}

impl From<JsonSyntaxError> for GeoJsonRejection {
    fn from(value: JsonSyntaxError) -> Self {
        Self::JsonSyntaxError(value)
    }
}

impl From<anyhow::Error> for GeoJsonRejection {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalError(value)
    }
}
