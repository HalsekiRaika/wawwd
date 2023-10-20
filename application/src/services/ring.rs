use crate::error::ApplicationError;
use crate::services::{
    CreateInstanceService, DependOnCreateInstanceService, DependOnUpdateInstanceService,
    UpdateInstanceService,
};
use crate::transfer::{CreateRingDto, RingDto};
use async_trait::async_trait;
use kernel::external::time::OffsetDateTime;
use kernel::{
    entities::{
        geology::Position,
        instance::{FinishedAt, Instance, InstanceId, RingSet, StartedAt},
        location::LocationId,
        ring::{CreatedAt, HueColor, Index, Ring, RingId, UserIp},
    },
    repository::{DependOnInstanceRepository, DependOnLocationRepository, InstanceRepository},
};
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait CreateRingService:
    'static
    + Sync
    + Send
    + DependOnLocationRepository
    + DependOnInstanceRepository
    + DependOnCreateInstanceService
    + DependOnUpdateInstanceService
{
    async fn create(&self, create: CreateRingDto) -> Result<RingDto, ApplicationError> {
        let CreateRingDto {
            location,
            longitude,
            latitude,
            indexed,
            hue,
            address,
            created_at,
        } = create;

        let instance = if let Some(instance) = self.instance_repository().find_unfinished().await? {
            instance
        } else {
            let id = InstanceId::default();
            let location = LocationId::new(location);
            let rings = RingSet::default();
            let started_at = StartedAt::default();
            let finished_at = FinishedAt::default();
            let instance = Instance::new(id, location, rings, started_at, finished_at);

            self.create_instance_service().create(instance).await?
        };

        let id = RingId::default();
        let pos = Position::new(longitude, latitude)?;
        let index = Index::new(indexed)?;
        let hue = HueColor::new(hue);
        let address = UserIp::new(address)?;
        let created_at = CreatedAt::new(created_at);

        let ring = Ring::new(id, pos, address, index, hue, created_at);

        let mut instance = instance.into_destruct();

        instance.rings.add(ring.clone())?;

        if instance.rings.len() > 70 {
            instance.finished_at = FinishedAt::new(OffsetDateTime::now_utc());
        }

        let instance = instance.freeze();
        let instance = self.update_instance_service().update(instance).await?;

        Ok((instance, ring).into())
    }
}
