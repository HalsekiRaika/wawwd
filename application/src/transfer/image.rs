use kernel::entities::image::{DestructImage, Image};
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;

pub struct ImageDto {
    pub id: Uuid,
    pub bin: String,
    pub created_at: OffsetDateTime,
}

impl From<Image> for ImageDto {
    fn from(value: Image) -> Self {
        let DestructImage {
            id,
            bin,
            created_at,
        } = value.into_destruct();
        Self {
            id: id.into(),
            bin: bin.into_base64(),
            created_at: created_at.into(),
        }
    }
}

pub struct CreateImageDto {
    pub id: Uuid,
    pub bin: String,
    pub created_at: OffsetDateTime,
}
