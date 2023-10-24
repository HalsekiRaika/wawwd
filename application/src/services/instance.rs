use crate::error::ApplicationError;
use async_trait::async_trait;
use kernel::entities::instance::Instance;
use kernel::repository::{
    DependOnInstanceRepository, DependOnLocationRepository, InstanceRepository, LocationRepository,
};
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait CreateInstanceService:
    'static + Sync + Send + DependOnLocationRepository + DependOnInstanceRepository
{
    async fn create(&self, create: Instance) -> Result<Instance, ApplicationError> {
        let Some(_) = self
            .location_repository()
            .find_by_id(create.location())
            .await?
        else {
            return Err(ApplicationError::NotFound {
                entity: "location",
                method: "find_by_id",
                target: create.location().to_string(),
            });
        };

        self.instance_repository().create(&create).await?;

        Ok(create)
    }
}

#[async_trait]
#[export_service]
pub trait UpdateInstanceService: 'static + Sync + Send + DependOnInstanceRepository {
    async fn update(&self, update: Instance) -> Result<Instance, ApplicationError> {
        let Some(before) = self.instance_repository().find_by_id(update.id()).await? else {
            return Err(ApplicationError::NotFound {
                entity: "instance",
                method: "find_by_id",
                target: update.id().to_string(),
            });
        };

        let mut before = before.into_destruct();
        let after = update.into_destruct();

        before.rings = after.rings;
        before.finished_at = after.finished_at;

        let instance = before.freeze();

        self.instance_repository().update(&instance).await?;

        Ok(instance)
    }
}