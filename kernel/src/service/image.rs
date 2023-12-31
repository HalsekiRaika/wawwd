use crate::entities::image::{Image, MetaData};
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait ImageExportExternalStorageService: 'static + Send + Sync {
    async fn export(&self, image: &Image, metadata: MetaData) -> Result<(), KernelError>;
}
