use async_trait::async_trait;
use orbital::export_service;
use crate::entities::volatiles::Etag;
use crate::error::KernelError;

#[async_trait]
#[export_service]
#[cfg_attr(feature = "mock", mockall::automock)]
pub trait LocationETagCache: 'static + Sync + Send {
    const NAMESPACE: &'static str = "location_etag_cache";
    async fn save(&self, tag: Etag) -> Result<(), KernelError>;
    async fn dele(&self) -> Result<(), KernelError>;
    async fn find(&self) -> Result<Option<Etag>, KernelError>;
}