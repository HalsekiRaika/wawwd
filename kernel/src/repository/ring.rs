use async_trait::async_trait;
use orbital::export_service;
use crate::entities::ring::{Ring, RingId};
use crate::error::KernelError;

#[async_trait]
#[export_service]
pub trait RingRepository: 'static + Sync + Send {
    async fn create(&self, create: &Ring) -> Result<(), KernelError>;
    async fn update(&self, update: &Ring) -> Result<(), KernelError>;
    async fn delete(&self, delete: &RingId) -> Result<(), KernelError>;
    async fn find_by_id(&self, id: &RingId) -> Result<Option<Ring>, KernelError>;
}
