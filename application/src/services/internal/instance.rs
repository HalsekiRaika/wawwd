use async_trait::async_trait;
use orbital::export_service;
use kernel::entities::instance::Instance;
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository, InstanceRepository, LocationRepository};
use crate::error::ApplicationError;

#[async_trait]
#[export_service]
pub trait CreateInstanceService: 'static + Sync + Send + DependOnLocationRepository + DependOnInstanceRepository {
    async fn create(&self, create: Instance) -> Result<Instance, ApplicationError> {
        let instance = create.into_destruct();
        
        let Some(_) = self.location_repository().find_by_id(&instance.location).await? else {
            return Err(ApplicationError::NotFound {
                entity: "location",
                method: "find_by_id",
                target: instance.location.to_string(),
            });
        };
        
        let instance = instance.freeze();
        
        self.instance_repository()
            .create(&instance)
            .await?;
        Ok(instance)
    }
}