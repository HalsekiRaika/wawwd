use serde::Deserialize;
use kernel::external::uuid::Uuid;

#[derive(Deserialize)]
pub struct RequireQuery {
    pub location: Uuid,
}
