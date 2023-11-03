use crate::error::ApplicationError;
use async_trait::async_trait;
use kernel::entities::instance::Instance;
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository, InstanceRepository};
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait CreateInstanceService:
    'static + Sync + Send + DependOnLocationRepository + DependOnInstanceRepository
{
    // noinspection DuplicatedCode
    async fn create(&self, create: Instance) -> Result<Instance, ApplicationError> {
        self.instance_repository().create(&create).await?;

        Ok(create)
    }
}

#[async_trait]
#[export_service]
pub trait CreateEmptyInstanceService:
    'static + Sync + Send + DependOnLocationRepository +  DependOnInstanceRepository
{
    // noinspection DuplicatedCode
    async fn create(&self) -> Result<Instance, ApplicationError> {
        let instance = Instance::default();
        self.instance_repository().create(&instance).await?;

        Ok(instance)
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

        let after = update.into_destruct();
        let instance = before.reconstruct(|dest| {
            dest.rings = after.rings;
            dest.finished_at = after.finished_at;
        });

        self.instance_repository().update(&instance).await?;

        Ok(instance)
    }
}
