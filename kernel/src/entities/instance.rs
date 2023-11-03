mod finished_at;
mod instance_id;
mod ring_set;
mod started_at;

pub use self::{finished_at::*, instance_id::*, ring_set::*, started_at::*};
use std::cmp::Ordering;
use destructure::Destructure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, Destructure)]
pub struct Instance {
    id: InstanceId,
    rings: RingSet,
    started_at: StartedAt,
    finished_at: FinishedAt,
}

impl Instance {
    pub fn new(
        id: InstanceId,
        rings: RingSet,
        started_at: StartedAt,
        finished_at: FinishedAt,
    ) -> Instance {
        Self {
            id,
            rings,
            started_at,
            finished_at,
        }
    }
}

impl Instance {
    pub fn id(&self) -> &InstanceId {
        &self.id
    }

    pub fn rings(&self) -> &RingSet {
        &self.rings
    }

    pub fn started_at(&self) -> &StartedAt {
        &self.started_at
    }

    pub fn finished_at(&self) -> &FinishedAt {
        &self.finished_at
    }
}

impl Eq for Instance {}

impl PartialEq<Self> for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.started_at.eq(&other.started_at)
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.started_at.cmp(&other.started_at)
    }
}

impl PartialOrd<Self> for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.started_at.partial_cmp(&other.started_at)
    }
}
