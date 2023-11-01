use async_trait::async_trait;
use orbital::export_service;
use crate::entities::volatiles::Etag;
use crate::error::KernelError;

#[async_trait]
#[export_service]
pub trait LocationETagCache: 'static + Sync + Send {
    const NAMESPACE: &'static str = "location_etag_cache";
    async fn save(&self, tag: Etag) -> Result<(), KernelError>;
}