mod instance_id;
mod ring_set;
mod laps;
mod started_at;
mod finished_at;

pub use self::{
    instance_id::*,
    laps::*,
    ring_set::*,
    started_at::*,
    finished_at::*,
};

use destructure::Destructure;
use serde::{Deserialize, Serialize};
use super::location::LocationId;


#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Instance {
    id: InstanceId,
    location: LocationId,
    laps: Laps,
    rings: RingSet,
    started_at: StartedAt,
    finished_at: FinishedAt,
}

impl Instance {
    pub fn new(
        id: InstanceId,
        location: LocationId,
        laps: Laps,
        rings: RingSet,
        started_at: StartedAt,
        finished_at: FinishedAt
    ) -> Instance {
        Self {
            id,
            location,
            laps,
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

    pub fn location(&self) -> &LocationId {
        &self.location
    }

    pub fn laps(&self) -> &Laps {
        &self.laps
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