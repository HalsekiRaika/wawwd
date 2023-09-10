use async_trait::async_trait;
use orbital::export_service;
use kernel::entities::geology::Position;
use kernel::entities::location::{LocalizedName, Location, LocationId};
use kernel::repository::{DependOnLocationRepository, LocationRepository};
use crate::error::ApplicationError;
use crate::transfer::{CreateLocationDto, DeleteLocationDto, LocationDto, UpdateLocationDto};

#[async_trait]
#[export_service]
pub trait CreateLocationService:
    'static
    + Send
    + Sync
    + DependOnLocationRepository
{
    async fn create(&self, create: CreateLocationDto) -> Result<LocationDto, ApplicationError> {
        let CreateLocationDto { latitude, longitude, localize } = create;

        let lid = LocationId::default();
        let pos = Position::new(longitude, latitude)?;
        let loc = localize.into_iter()
            .map(|(c, n)| LocalizedName::new(c, n))
            .collect::<Vec<LocalizedName>>();

        let mark = Location::new(lid, pos, loc);

        self.location_repository()
            .create(&mark)
            .await?;

        Ok(mark.into())
    }
}


#[async_trait]
#[export_service]
pub trait UpdateLocationService:
    'static
    + Send
    + Sync
    + DependOnLocationRepository
{
    //noinspection DuplicatedCode
    async fn update(&self, update: UpdateLocationDto) -> Result<LocationDto, ApplicationError> {
        let UpdateLocationDto { id, latitude, longitude, localize } = update;

        let lid = LocationId::new(id);
        let Some(mark) = self.location_repository().find_by_id(&lid).await? else {
            return Err(ApplicationError::NotFound {
                entity: "location",
                method: "update",
                target: lid.to_string()
            })
        };

        let mut mark = mark.into_destruct();

        mark.pos = Position::new(longitude, latitude)?;
        mark.localize = localize.into_iter()
            .map(|(c, n)| LocalizedName::new(c, n))
            .collect::<Vec<LocalizedName>>();

        let mark = mark.freeze();

        self.location_repository()
            .update(&mark)
            .await?;

        Ok(mark.into())
    }
}


#[async_trait]
#[export_service]
pub trait DeleteLocationService:
    'static
    + Send
    + Sync
    + DependOnLocationRepository
{
    //noinspection DuplicatedCode
    async fn delete(&self, delete: DeleteLocationDto) -> Result<(), ApplicationError> {
        let DeleteLocationDto { id } = delete;

        let lid = LocationId::new(id);
        let Some(mark) = self.location_repository().find_by_id(&lid).await? else {
            return Err(ApplicationError::NotFound {
                entity: "location",
                method: "delete",
                target: lid.to_string()
            })
        };

        self.location_repository()
            .delete(mark.id())
            .await?;

        Ok(())
    }
}