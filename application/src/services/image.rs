use crate::error::ApplicationError;
use crate::transfer::{CreateImageDto, ImageDto};
use async_trait::async_trait;
use kernel::entities::image::{Image, ImageBin};
use kernel::entities::ring::{CreatedAt, RingId};
use kernel::finder::{DependOnRingFinder, RingFinder};
use kernel::repository::{DependOnImageRepository, ImageRepository};
use kernel::service::{
    DependOnImageExportExternalStorageService, ImageExportExternalStorageService,
};
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait ExportImageService:
    'static
    + Sync
    + Send
    + DependOnRingFinder
    + DependOnImageRepository
    + DependOnImageExportExternalStorageService
{
    async fn export(&self, export: CreateImageDto) -> Result<ImageDto, ApplicationError> {
        let CreateImageDto {
            id,
            bin,
            created_at,
        } = export;

        let id = RingId::new(id);

        let Some(_) = self.ring_finder().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                entity: "ring",
                method: "find_by_id",
                target: id.to_string(),
            });
        };

        let binary = ImageBin::from_base64(bin)?;
        let created_at = CreatedAt::new(created_at);
        let image = Image::new(id, binary, created_at);

        self.image_repository().create(&image).await?;

        if let Err(e) = self.image_export_external_storage_service().export(&image).await {
            self.image_repository().delete(image.id()).await?;
            return Err(e.into());
        }

        Ok(image.into())
    }
}
