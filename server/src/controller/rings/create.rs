use serde::{Deserialize, Serialize};
use application::transfer::{CreateRingDto, RingDto};
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use crate::controller::{Exhaust, Intake};

pub struct RequestToCreateRingDto;

impl Intake<CreateRingRequest> for RequestToCreateRingDto {
    type To = CreateRingDto;
    fn emit(&self, input: CreateRingRequest) -> Self::To {
        CreateRingDto {
            location: input.location,
            longitude: input.longitude,
            latitude: input.latitude,
            indexed: input.indexed,
            hue: input.hue,
            address: input.address,
            created_at: input.created_at,
        }
    }
}

pub struct RingDtoToResponseJson;

impl Exhaust<RingDto> for RingDtoToResponseJson {
    type To = RingResponse;
    fn emit(&self, input: RingDto) -> Self::To {
        RingResponse {
            id: input.id,
            instance: input.instance,
            location: input.location,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRingRequest {
    pub location: Uuid,
    pub longitude: f64,
    pub latitude: f64,
    pub indexed: i32,
    pub hue: i32,
    pub address: String,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize)]
pub struct RingResponse {
    id: Uuid,
    instance: Uuid,
    location: Uuid,
}