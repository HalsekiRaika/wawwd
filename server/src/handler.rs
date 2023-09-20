use crate::error::ServerError;
use application::services::{
    DependOnCreateLocationService, DependOnDeleteLocationService, DependOnUpdateLocationService,
};
use driver::database::LocationDataBase;
use driver::DataBaseInitializer;
use kernel::repository::DependOnLocationRepository;

#[derive(Clone)]
pub struct Handler {
    loc: LocationDataBase,
}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        let pg_url = dotenvy::var("PG_DATABASE_URL")
            .map_err(|_| ServerError::EnvError(r#"PG_DATABASE_URL"#))?;

        let pg_pool = DataBaseInitializer::setup(pg_url).await?;

        let loc = LocationDataBase::new(pg_pool);

        Ok(Self { loc })
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
