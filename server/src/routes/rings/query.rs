use serde::Deserialize;
use kernel::external::uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SelectionQuery {
    pub id: Option<Uuid>
}