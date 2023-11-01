use crate::controller::{Exhaust, Intake};
use kernel::entities::instance::{Instance, InstanceId};
use kernel::entities::ring::Ring;
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use kernel::entities::location::LocationId;

pub struct SelectionIdToInstanceId;

impl Intake<Uuid> for SelectionIdToInstanceId {
    type To = InstanceId;
    fn emit(&self, input: Uuid) -> Self::To {
        InstanceId::new(input)
    }
}

pub struct SelectionIdToLocationId;

impl Intake<Uuid> for SelectionIdToLocationId {
    type To = LocationId;
    fn emit(&self, input: Uuid) -> Self::To {
        LocationId::new(input)
    }
}


pub struct InstanceToDetailResponse;

impl Exhaust<Instance> for InstanceToDetailResponse {
    type To = RingInstanceWithDetail;
    fn emit(&self, input: Instance) -> Self::To {
        input.into()
    }
}

pub struct MaybeInstanceToDetailResponse;

impl Exhaust<Option<Instance>> for MaybeInstanceToDetailResponse {
    type To = Option<RingInstanceWithDetail>;
    fn emit(&self, input: Option<Instance>) -> Self::To {
        input.map(|ins| ins.into())
    }
}

pub struct InstancesToJsonBTreeSet;

impl Exhaust<BTreeSet<Instance>> for InstancesToJsonBTreeSet {
    type To = BTreeSet<RingInstance>;

    fn emit(&self, input: BTreeSet<Instance>) -> Self::To {
        input.into_iter().map(Into::into).collect()
    }
}

#[derive(Debug, Serialize)]
pub struct RingInstanceWithDetail {
    id: Uuid,
    location: Uuid,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    started_at: OffsetDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "kernel::external::time::serde::iso8601::option")]
    finished_at: Option<OffsetDateTime>,
    rings: BTreeSet<Ring>,
}

impl From<Instance> for RingInstanceWithDetail {
    fn from(value: Instance) -> Self {
        let value = value.into_destruct();
        RingInstanceWithDetail {
            id: value.id.into(),
            location: value.location.into(),
            started_at: value.started_at.into(),
            finished_at: value.finished_at.into(),
            rings: value.rings.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RingInstance {
    id: Uuid,
    location: Uuid,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    started_at: OffsetDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "kernel::external::time::serde::iso8601::option")]
    finished_at: Option<OffsetDateTime>,
}

impl From<Instance> for RingInstance {
    fn from(value: Instance) -> Self {
        let value = value.into_destruct();
        Self {
            id: value.id.into(),
            location: value.location.into(),
            started_at: value.started_at.into(),
            finished_at: value.finished_at.into(),
        }
    }
}

impl Eq for RingInstance {}

impl PartialEq<Self> for RingInstance {
    fn eq(&self, other: &Self) -> bool {
        self.started_at.eq(&other.started_at)
    }
}

impl Ord for RingInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.started_at.cmp(&other.started_at)
    }
}

impl PartialOrd<Self> for RingInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.started_at.partial_cmp(&other.started_at)
    }
}
