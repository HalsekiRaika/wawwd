use crate::error::ApplicationError;
use crate::transfer::{CreateImageDto, ImageDto};
use async_trait::async_trait;
use kernel::entities::image::{Image, ImageBin, MetaData};
use kernel::entities::ring::{CreatedAt, DestructRing, RingId};
use kernel::finder::{DependOnRingFinder, RingFinder};
use kernel::repository::{DependOnImageRepository, DependOnLocationRepository, ImageRepository, LocationRepository};
use kernel::service::{
    DependOnImageExportExternalStorageService, ImageExportExternalStorageService,
};
use orbital::export_service;
use kernel::entities::location::LocalizeId;

#[async_trait]
#[export_service]
pub trait ExportImageService:
    'static
    + Sync
    + Send
    + DependOnRingFinder
    + DependOnLocationRepository
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

        let Some(ring) = self.ring_finder().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                entity: "ring",
                method: "find_by_id",
                target: id.to_string(),
            });
        };

        let binary = ImageBin::from_base64(bin)?;
        let created_at = CreatedAt::new(created_at);
        let image = Image::new(id, binary, created_at);
        let DestructRing {
            id,
            location,
            indexed,
            hue,
            created_at,
            ..
        } = ring.into_destruct();

        let Some(location) = self.location_repository().find_by_id(&location).await? else {
            return Err(ApplicationError::NotFound {
                entity: "location",
                method: "find_by_id",
                target: location.to_string(),
            });
        };

        let dest = location.into_destruct();
        let Some(localize) = dest.localize.into_iter()
            .find(|loc| loc.country() == &LocalizeId::unchecked_new("en"))
            .map(|loc| loc.into_destruct())
            .map(|loc| loc.localize)
        else {
            return Err(ApplicationError::NotFound {
                entity: "localize",
                method: "find_by_id",
                target: format!("location `{}`, in `en` localize.", dest.id),
            })
        };

        let metadata = MetaData::new(id, indexed, hue, localize, created_at);

        self.image_repository().create(&image).await?;

        if let Err(e) = self.image_export_external_storage_service().export(&image, metadata).await {
            self.image_repository().delete(image.id()).await?;
            return Err(e.into());
        }

        Ok(image.into())
    }
}
