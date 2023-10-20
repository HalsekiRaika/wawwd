mod query;

use std::collections::BTreeSet;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use application::services::{CreateRingService, DependOnCreateRingService};
use kernel::repository::{DependOnInstanceRepository, InstanceRepository};
use crate::controller::{Controller, CreateRingRequest, InstancesToJsonBTreeSet, InstanceToDetailResponse, RequestToCreateRingDto, RingDtoToResponseJson, RingInstance, RingInstanceWithDetail, SelectionIdToInstanceId};
use crate::error::ServerError;
use crate::Handler;
use self::query::SelectionQuery;

pub async fn rings(
    State(handler): State<Handler>,
    Query(query): Query<SelectionQuery>
) -> Result<impl IntoResponse, ServerError> {
    #[derive(Serialize)]
    #[serde(untagged)]
    enum RetType {
        Detail(RingInstanceWithDetail),
        Set(BTreeSet<RingInstance>)
    }


    let res = if let Some(id) = query.id {
        let res = Controller::new(SelectionIdToInstanceId, InstanceToDetailResponse)
            .intake(id)
            .handle(|input| async move {
                handler.instance_repository()
                    .find_by_id(&input)
                    .await
            })
            .await?;
        RetType::Detail(res.ok_or(ServerError::NotFound {
            entity: "Instance (with Details)",
            target: id.to_string(),
        })?)
    } else {
        let res = Controller::new((), InstancesToJsonBTreeSet)
            .bypass(|| async {
                handler.instance_repository().find_all().await
            })
            .await?;
        RetType::Set(res)
    };

    Ok(Json(res))
}


pub async fn reg_ring(
    State(handler): State<Handler>,
    Json(ctx): Json<CreateRingRequest>
) -> Result<impl IntoResponse, ServerError> {
    let res = Controller::new(RequestToCreateRingDto, RingDtoToResponseJson)
        .intake(ctx)
        .handle(|input| async {
            handler.create_ring_service()
                .create(input)
                .await
        })
        .await?;
    Ok((StatusCode::CREATED, Json(res)))
}