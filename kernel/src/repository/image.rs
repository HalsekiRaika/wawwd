use crate::entities::image::Image;
use crate::entities::ring::RingId;
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait ImageRepository: 'static + Send + Sync {
    async fn create(&self, image: &Image) -> Result<(), KernelError>;
    async fn delete(&self, id: &RingId) -> Result<(), KernelError>;
    async fn find_by_id(&self, id: &RingId) -> Result<Option<Image>, KernelError>;
}
