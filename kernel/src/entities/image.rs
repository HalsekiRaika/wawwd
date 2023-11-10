mod binary;
mod metadata;
mod trait_tag;

pub use self::binary::*;
pub use self::metadata::*;
pub use self::trait_tag::*;

use destructure::Destructure;
use serde::{Deserialize, Serialize};

use crate::entities::ring::{CreatedAt, RingId};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure)]
pub struct Image {
    id: RingId,
    bin: ImageBin,
    created_at: CreatedAt,
}

impl Image {
    pub fn new(id: RingId, bin: ImageBin, created_at: CreatedAt) -> Self {
        Self {
            id,
            bin,
            created_at,
        }
    }
}

impl Image {
    pub fn id(&self) -> &RingId {
        &self.id
    }

    pub fn bin(&self) -> &ImageBin {
        &self.bin
    }

    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }
}
