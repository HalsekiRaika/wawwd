use kernel::external::uuid::Uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SelectionQuery {
    pub id: Option<Uuid>,
}
