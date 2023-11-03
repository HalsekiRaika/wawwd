mod created_at;
mod hue;
mod index;
mod ring_id;
mod user_id;

pub use self::{created_at::*, hue::*, index::*, ring_id::*, user_id::*};

use crate::entities::geology::Position;
use crate::entities::location::LocationId;
use destructure::Destructure;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Ring {
    id: RingId,
    pos_in: Position,
    location: LocationId,
    user: UserId,
    indexed: Index,
    hue: HueColor,
    created_at: CreatedAt,
}

impl Ring {
    pub fn new(
        id: RingId,
        pos_in: Position,
        location: LocationId,
        user: UserId,
        indexed: Index,
        hue: HueColor,
        created_at: CreatedAt,
    ) -> Ring {
        Self {
            id,
            pos_in,
            location,
            user,
            indexed,
            hue,
            created_at,
        }
    }
}

impl Ring {
    pub fn id(&self) -> &RingId {
        &self.id
    }

    pub fn pos_in(&self) -> &Position {
        &self.pos_in
    }

    pub fn location(&self) -> &LocationId {
        &self.location
    }

    pub fn user(&self) -> &UserId {
        &self.user
    }

    pub fn indexed(&self) -> &Index {
        &self.indexed
    }

    pub fn hue(&self) -> &HueColor {
        &self.hue
    }

    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }
}

impl Eq for Ring {}

impl PartialEq<Self> for Ring {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialEq<Index> for Ring {
    fn eq(&self, other: &Index) -> bool {
        self.indexed.eq(other)
    }
}

impl PartialEq<CreatedAt> for Ring {
    fn eq(&self, other: &CreatedAt) -> bool {
        self.created_at.eq(other)
    }
}

impl PartialEq<UserId> for Ring {
    fn eq(&self, other: &UserId) -> bool {
        self.user.eq(other)
    }
}

impl Ord for Ring {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialOrd<Self> for Ring {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.created_at.partial_cmp(&other.created_at)
    }
}
