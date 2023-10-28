use crate::error::ServerError;
use crate::AppHandler;
use axum::extract::State;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::TypedHeader;
use kernel::entities::token::AdminToken;
use kernel::security::{AuthorizeAdminPolicy, DependOnAuthorizeAdminPolicy};

pub async fn simple_auth<B>(
    State(handler): State<AppHandler>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, ServerError> {
    handler
        .authorize_admin_policy()
        .authorize(&AdminToken::new(auth.token()))
        .await
        .map_err(ServerError::UnAuthorize)?;

    let res = next.run(request).await;
    Ok(res)
}
