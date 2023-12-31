use crate::controller::{Exhaust, Intake};
use application::transfer::{CreateRingDto, RingDto};
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use serde::{Deserialize, Serialize};

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
            user: input.user,
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

pub struct RingDtoToDetailResponseJson;

impl Exhaust<RingDto> for RingDtoToDetailResponseJson {
    type To = RingDetailResponse;
    fn emit(&self, input: RingDto) -> Self::To {
        RingDetailResponse {
            id: input.id,
            instance: input.instance,
            location: input.location,
            indexed: input.indexed,
            hue: input.hue,
            user: input.user,
            created_at: input.created_at,
            nonce: None
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
    pub user: Uuid,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateRingRequestWithNonce {
    #[serde(flatten)]
    pub req: CreateRingRequest,
    #[serde(default)]
    pub nonce: Option<Uuid>
}

#[derive(Serialize)]
pub struct RingResponse {
    id: Uuid,
    instance: Uuid,
    location: Uuid,
}

#[derive(Serialize)]
pub struct RingDetailResponse {
    id: Uuid,
    instance: Uuid,
    location: Uuid,
    indexed: i32,
    hue: i32,
    user: Uuid,
    #[serde(with = "kernel::external::time::serde::iso8601")]
    created_at: OffsetDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Uuid>
}