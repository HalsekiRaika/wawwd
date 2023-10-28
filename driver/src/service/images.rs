use crate::error::DriverError;
use async_trait::async_trait;
use kernel::entities::image::Image;
use kernel::error::KernelError;
use kernel::service::ImageExportExternalStorageService;
use s3::Bucket;

pub struct S3ImageStorageService {
    bucket: Bucket,
}

impl S3ImageStorageService {
    pub fn new(bucket: Bucket) -> Self {
        Self { bucket }
    }
}

#[async_trait]
impl ImageExportExternalStorageService for S3ImageStorageService {
    async fn export(&self, target: &Image) -> Result<(), KernelError> {
        S3ImageStorageServiceInternalProcessor::export(target, &self.bucket)
            .await
            .map_err(DriverError::from)?;
        Ok(())
    }
}

pub struct S3ImageStorageServiceInternalProcessor;

impl S3ImageStorageServiceInternalProcessor {
    pub async fn export(target: &Image, buc: &Bucket) -> Result<(), DriverError> {
        buc.put_object(
            format!("/{}.png", target.created_at()),
            target.bin().as_ref(),
        )
        .await?;
        Ok(())
    }
}
