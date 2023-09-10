mod location_id;
mod localized_name;

pub use self::{
    location_id::*,
    localized_name::*
};

use destructure::Destructure;
use serde::{Deserialize, Serialize};

use super::geology::Position;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Location {
    id: LocationId,
    pos: Position,
    localize: Vec<LocalizedName>,
}

impl Location {
    pub fn new(id: LocationId, pos: Position, localize: Vec<LocalizedName>) -> Location {
        Self { id, pos, localize }
    }

    pub fn id(&self) -> &LocationId {
        &self.id
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn localize(&self) -> &[LocalizedName] {
        &self.localize
    }
}
