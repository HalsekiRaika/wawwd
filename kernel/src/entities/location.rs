mod localize_id;
mod localized_name;
mod location_id;

pub use self::{localize_id::*, localized_name::*, location_id::*};

use destructure::Destructure;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::KernelError;

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

    pub fn r#try(
        id: impl Into<Uuid>,
        pos: impl TryInto<Position, Error=KernelError>,
        localize: impl Into<Vec<LocalizedName>>
    ) -> Result<Location, KernelError> {
        Ok(Self {
            id: LocationId::new(id),
            pos: pos.try_into()?,
            localize: localize.into()
        })
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
