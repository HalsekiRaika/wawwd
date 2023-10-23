use crate::error::ServerError;
use application::services::{
    DependOnCreateInstanceService, DependOnCreateLocationService, DependOnCreateRingService,
    DependOnDeleteLocationService, DependOnUpdateInstanceService, DependOnUpdateLocationService,
};
use driver::database::{InstanceDataBase, LocationDataBase};
use driver::DataBaseInitializer;
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};

#[derive(Clone)]
pub struct Handler {
    loc: LocationDataBase,
    ins: InstanceDataBase,
}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        let pg_url = dotenvy::var("PG_DATABASE_URL")
            .map_err(|_| ServerError::EnvError(r#"PG_DATABASE_URL"#))?;

        let pg_pool = DataBaseInitializer::setup(pg_url).await?;

        let loc = LocationDataBase::new(pg_pool.clone());
        let ins = InstanceDataBase::new(pg_pool);

        Ok(Self { loc, ins })
    }
}

impl DependOnLocationRepository for Handler {
    type LocationRepository = LocationDataBase;
    fn location_repository(&self) -> &Self::LocationRepository {
        &self.loc
    }
}

impl DependOnCreateLocationService for Handler {
    type CreateLocationService = Self;
    fn create_location_service(&self) -> &Self::CreateLocationService {
        self
    }
}

impl DependOnUpdateLocationService for Handler {
    type UpdateLocationService = Self;
    fn update_location_service(&self) -> &Self::UpdateLocationService {
        self
    }
}

impl DependOnDeleteLocationService for Handler {
    type DeleteLocationService = Self;

    fn delete_location_service(&self) -> &Self::DeleteLocationService {
        self
    }
}

impl DependOnInstanceRepository for Handler {
    type InstanceRepository = InstanceDataBase;
    fn instance_repository(&self) -> &Self::InstanceRepository {
        &self.ins
    }
}

impl DependOnCreateRingService for Handler {
    type CreateRingService = Self;
    fn create_ring_service(&self) -> &Self::CreateRingService {
        self
    }
}

impl DependOnCreateInstanceService for Handler {
    type CreateInstanceService = Self;
    fn create_instance_service(&self) -> &Self::CreateInstanceService {
        self
    }
}

impl DependOnUpdateInstanceService for Handler {
    type UpdateInstanceService = Self;
    fn update_instance_service(&self) -> &Self::UpdateInstanceService {
        self
    }
}
