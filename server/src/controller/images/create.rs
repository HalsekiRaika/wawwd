use crate::controller::Intake;
use application::transfer::CreateImageDto;
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateImageRequest {
    pub ring_id: Uuid,
    pub image: String,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

pub struct CreateImageRequestToCreateImageDto;

impl Intake<CreateImageRequest> for CreateImageRequestToCreateImageDto {
    type To = CreateImageDto;
    fn emit(&self, input: CreateImageRequest) -> Self::To {
        CreateImageDto {
            id: input.ring_id,
            bin: input.image,
            created_at: input.created_at,
        }
    }
}
