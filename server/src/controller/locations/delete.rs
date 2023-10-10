use crate::controller::Intake;
use application::transfer::DeleteLocationDto;
use kernel::external::uuid::Uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    id: Uuid,
    localize: Option<String>,
}

pub struct DeleteRequestToDeleteLocationDto;

impl Intake<DeleteRequest> for DeleteRequestToDeleteLocationDto {
    type To = DeleteLocationDto;

    fn emit(&self, input: DeleteRequest) -> Self::To {
        DeleteLocationDto {
            id: input.id,
            localize: input.localize,
        }
    }
}
