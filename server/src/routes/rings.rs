mod query;

use std::collections::BTreeSet;
use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use kernel::repository::{DependOnInstanceRepository, InstanceRepository};
use crate::controller::{Controller, CreateRingRequest, InstancesToJsonBTreeSet, SelectionIdToInstanceId};
use crate::error::ServerError;
use crate::Handler;
use self::query::SelectionQuery;

pub async fn rings(
    State(handler): State<Handler>,
    Query(query): Query<SelectionQuery>
) -> Result<impl IntoResponse, ServerError> {
    #[derive(Serialize)]
    #[serde(untagged)]
    enum RetType<T> {
        Option(Option<T>),
        BTreeSet(BTreeSet<T>),
    }

    let ret = match query.id {
        Some(id) => {
            let found = Controller::new(SelectionIdToInstanceId, ())
                .intake(id)
                .bypass(|input| async move {
                    handler.instance_repository()
                        .find_by_id(&input)
                        .await
                })
                .await?;
            RetType::Option(found)
        }
        None => {
            let all = Controller::new((), InstancesToJsonBTreeSet)
                .bypass(|| async {
                    handler.instance_repository()
                        .find_all()
                        .await
                })
                .await?;
            RetType::BTreeSet(all)
        }
    };

    Ok(Json(ret))
}


pub async fn reg_ring(
    State(handler): State<Handler>,
    Json(ctx): Json<CreateRingRequest>
) -> Result<impl IntoResponse, ServerError> {

    Ok(())
}