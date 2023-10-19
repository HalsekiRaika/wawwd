use serde::Deserialize;
use application::transfer::CreateRingDto;
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use crate::controller::Intake;

pub struct RequestToCreateRingDto;

impl Intake<CreateRingRequest> for RequestToCreateRingDto {
    type To = CreateRingDto;
    fn emit(&self, input: CreateRingRequest) -> Self::To {
        todo!()
    }
}

#[derive(Deserialize)]
pub struct CreateRingRequest {
    pub instance: Option<Uuid>,
    pub location: Uuid,
    pub longitude: i32,
    pub latitude: i32,
    pub indexed: i32,
    pub hue: i32,
    pub address: String,
    pub created_at: OffsetDateTime,
}