use destructure::Destructure;
use serde::{Deserialize, Serialize};
use crate::entities::location::LocalizeName;
use crate::entities::ring::{CreatedAt, HueColor, Index, RingId};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct MetaData {
    ring_id: RingId,
    index: Index,
    color: HueColor,
    location: LocalizeName,
    creation_date: CreatedAt
}

impl MetaData {
    pub fn new(
        ring_id: RingId,
        index: Index,
        color: HueColor,
        location: LocalizeName,
        creation_date: CreatedAt
    ) -> Self {
        Self {
            ring_id,
            index,
            color,
            location,
            creation_date
        }
    }
}