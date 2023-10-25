use std::ops::Deref;
use std::sync::Arc;
use crate::error::ServerError;
use application::services::{
    DependOnCreateInstanceService, DependOnCreateLocationService, DependOnCreateRingService,
    DependOnDeleteLocationService, DependOnUpdateInstanceService, DependOnUpdateLocationService,
};
use driver::database::{InstanceDataBase, LocationDataBase};
use driver::DataBaseInitializer;
use driver::security::AuthorizeInMemoryInstance;
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};
use kernel::security::DependOnAuthorizeAdminPolicy;

pub struct AppHandler {
    inner: Arc<Handler>,
}

impl AppHandler {
    pub async fn init() -> Result<AppHandler, ServerError> {
        Ok(Self { inner: Arc::new(Handler::init().await?) })
    }
}

impl Clone for AppHandler {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Deref for AppHandler {
    type Target = Handler;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

pub struct Handler {
    loc: LocationDataBase,
    ins: InstanceDataBase,
    auth: AuthorizeInMemoryInstance,
}

impl Handler {
    #[allow(dead_code)]
    async fn init() -> Result<Self, ServerError> {
        let pg_url = dotenvy::var("PG_DATABASE_URL")
            .map_err(|_| ServerError::EnvError(r#"PG_DATABASE_URL"#))?;

        let one_time = kernel::entities::token::AdminToken::default();
        tracing::info!("+ Admin Token generated.");
        tracing::info!("| * {:?}", one_time);
        tracing::info!("| This token is available as long as this instance is active,");
        tracing::info!("+ but is immediately discarded and a new token is regenerated when it is restarted.");

        let pg_pool = DataBaseInitializer::setup(pg_url).await?;

        let loc = LocationDataBase::new(pg_pool.clone());
        let ins = InstanceDataBase::new(pg_pool);
        let auth = AuthorizeInMemoryInstance::new(one_time);

        Ok(Self { loc, ins, auth })
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

impl DependOnAuthorizeAdminPolicy for Handler {
    type AuthorizeAdminPolicy = AuthorizeInMemoryInstance;
    fn authorize_admin_policy(&self) -> &Self::AuthorizeAdminPolicy {
        &self.auth
    }
}