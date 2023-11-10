use crate::error::DriverError;
use async_trait::async_trait;
use kernel::entities::image::{DestructMetaData, Image, MetaData, TraitData};
use kernel::error::KernelError;
use kernel::service::ImageExportExternalStorageService;
use s3::Bucket;
use serde_json::json;
use kernel::entities::ring::CreatedAt;
use kernel::external::uuid::Uuid;

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
    async fn export(&self, target: &Image, metadata: MetaData) -> Result<(), KernelError> {
        S3ImageStorageServiceInternalProcessor::export(target, metadata, &self.bucket)
            .await
            .map_err(DriverError::from)?;
        Ok(())
    }
}

pub struct S3ImageStorageServiceInternalProcessor;

impl S3ImageStorageServiceInternalProcessor {
    pub async fn export(target: &Image, metadata: MetaData, buc: &Bucket) -> Result<(), DriverError> {
        buc.put_object(
            format!("/{}.png", target.created_at()),
            target.bin().as_ref(),
        )
        .await?;

        let DestructMetaData {
            ring_id,
            index,
            color,
            location,
            creation_date
        } = metadata.into_destruct();

        let ring_id: TraitData<Uuid> = TraitData::from(ring_id);
        let index: TraitData<i32> = TraitData::from(index);
        let color: TraitData<i32> = TraitData::from(color);
        let location: TraitData<String> = TraitData::from(location);
        let creation_date: TraitData<CreatedAt> = TraitData::from(creation_date);
        let json = json!({
            "name": format!("wawwd-{}", target.created_at()),
            "description": "wawwd AR app photo",
            "image": format!("{}.png", target.created_at()),
            "attributes": [
                ring_id,
                index,
                color,
                location,
                creation_date
            ]
        });

        buc.put_object(
            format!("/{}.meta.json", target.created_at()),
            json.to_string().as_bytes()
        )
        .await?;

        Ok(())
    }
}
