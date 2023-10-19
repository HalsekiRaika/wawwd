use std::cmp::Ordering;
use std::collections::BTreeSet;
use serde::Serialize;
use kernel::entities::instance::{Instance, InstanceId};
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use crate::controller::{Exhaust, Intake};

pub struct SelectionIdToInstanceId;

impl Intake<Uuid> for SelectionIdToInstanceId {
    type To = InstanceId;
    fn emit(&self, input: Uuid) -> Self::To {
        InstanceId::new(input)
    }
}

pub struct InstancesToJsonBTreeSet;

impl Exhaust<BTreeSet<Instance>> for InstancesToJsonBTreeSet {
    type To = BTreeSet<RingInstance>;

    fn emit(&self, input: BTreeSet<Instance>) -> Self::To {
        input.into_iter()
            .map(Into::into)
            .collect()
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
    finished_at: Option<OffsetDateTime>
}

impl From<Instance> for RingInstance {
    fn from(value: Instance) -> Self {
        let value = value.into_destruct();
        Self {
            id: value.id.into(),
            location: value.location.into(),
            started_at: value.started_at.into(),
            finished_at: value.finished_at.into()
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
