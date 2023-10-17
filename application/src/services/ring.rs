use async_trait::async_trait;
use orbital::export_service;
use kernel::{
    entities::{geology::Position, instance::{RingSet, InstanceId, Instance, FinishedAt, StartedAt}, location::LocationId, ring::{Index, HueColor, CreatedAt, Ring, RingId, UserIp}},
    repository::{DependOnInstanceRepository, DependOnLocationRepository, InstanceRepository, LocationRepository, DependOnRingRepository}
};
use kernel::repository::RingRepository;
use crate::error::ApplicationError;
use crate::services::{CreateInstanceService, DependOnCreateInstanceService};
use crate::transfer::{CreateRingDto, RingDto};

#[async_trait]
#[export_service]
pub trait CreateRingService:
    'static
    + Sync
    + Send
    + DependOnRingRepository
    + DependOnLocationRepository
    + DependOnInstanceRepository
    + DependOnCreateInstanceService
{
    async fn create(&self, create: CreateRingDto) -> Result<RingDto, ApplicationError> {
        let CreateRingDto {
            instance,
            location,
            longitude,
            latitude,
            indexed,
            hue,
            address,
            created_at
        } = create;

        let instance = if let Some(id) = instance.map(InstanceId::new) {
            self.instance_repository()
                .find_by_id(&id)
                .await?
                .ok_or(ApplicationError::NotFound {
                    entity: "instance",
                    target: id.to_string(),
                    method: "find_by_id"
                })?
        } else {
            let id = InstanceId::default();
            let location = LocationId::new(location);
            let rings = RingSet::default();
            let started_at = StartedAt::default();
            let finished_at = FinishedAt::default();
            let instance = Instance::new(id, location, rings, started_at, finished_at);

            self.create_instance_service()
                .create(instance)
                .await?
        };

        let id = RingId::default();
        let pos = Position::new(longitude, latitude)?;
        let index = Index::new(indexed)?;
        let hue = HueColor::new(hue);
        let address = UserIp::new(address)?;
        let created_at = CreatedAt::new(created_at);

        let ring = Ring::new(id, pos, address, index, hue, created_at);

        instance.rings().valid(&ring)?;
        self.ring_repository()
            .create(&ring)
            .await?;

        Ok((instance, ring).into())
    }
}