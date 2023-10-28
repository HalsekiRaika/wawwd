use crate::entities::ring::{Ring, RingId};
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait RingFinder: 'static + Sync + Send {
    async fn find_by_id(&self, id: &RingId) -> Result<Option<Ring>, KernelError>;
}
