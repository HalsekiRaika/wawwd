use crate::entities::location::{LocalizeId, Location, LocationId};
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[rustfmt::skip]
#[async_trait]
#[export_service]
#[cfg_attr(feature = "mock", mockall::automock)]
pub trait LocationRepository: 'static + Send + Sync {
    async fn create(&self, create: &Location) -> Result<(), KernelError>;
    async fn update(&self, update: &Location) -> Result<(), KernelError>;
    async fn delete(&self, delete: &LocationId) -> Result<(), KernelError>;
    async fn delete_localize(&self, delete: &LocationId, code: &LocalizeId) -> Result<(), KernelError>;
    async fn find_all(&self) -> Result<Vec<Location>, KernelError>;
    async fn find_by_id(&self, id: &LocationId) -> Result<Option<Location>, KernelError>;
}