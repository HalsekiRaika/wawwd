mod query;

use self::query::SelectionQuery;
use crate::controller::{
    Controller, CreateRingRequest, MaybeInstanceToDetailResponse, InstancesToJsonBTreeSet,
    RequestToCreateRingDto, RingDtoToResponseJson, RingInstance, RingInstanceWithDetail,
    SelectionIdToInstanceId,
};
use crate::error::ServerError;
use crate::AppHandler;
use application::services::{CreateRingService, DependOnCreateRingService};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use kernel::repository::{DependOnInstanceRepository, InstanceRepository};
use serde::Serialize;
use std::collections::BTreeSet;

pub async fn rings(
    State(handler): State<AppHandler>,
    Query(query): Query<SelectionQuery>,
) -> Result<impl IntoResponse, ServerError> {
    #[derive(Serialize)]
    #[serde(untagged)]
    enum RetType {
        Detail(RingInstanceWithDetail),
        Set(BTreeSet<RingInstance>),
    }

    let res = if let Some(id) = query.id {
        let res = Controller::new(SelectionIdToInstanceId, MaybeInstanceToDetailResponse)
            .intake(id)
            .handle(|input| async move { handler.instance_repository().find_by_id(&input).await })
            .await?;
        RetType::Detail(res.ok_or(ServerError::NotFound {
            entity: "Instance (with Details)",
            target: id.to_string(),
        })?)
    } else {
        let res = Controller::new((), InstancesToJsonBTreeSet)
            .bypass(|| async { handler.instance_repository().find_all().await })
            .await?;
        RetType::Set(res)
    };

    Ok(Json(res))
}

pub async fn reg_ring(
    State(handler): State<AppHandler>,
    Json(ctx): Json<CreateRingRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let res = Controller::new(RequestToCreateRingDto, RingDtoToResponseJson)
        .intake(ctx)
        .handle(|input| async { handler.create_ring_service().create(input).await })
        .await?;
    Ok((StatusCode::CREATED, Json(res)))
}
