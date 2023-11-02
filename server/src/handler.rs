use std::ops::Deref;
use crate::error::ServerError;
use application::services::{DependOnCreateEmptyInstanceService, DependOnCreateInstanceService, DependOnCreateLocationService, DependOnCreateRingService, DependOnDeleteLocationService, DependOnExportImageService, DependOnUpdateInstanceService, DependOnUpdateLocationService};
use driver::database::{ImageDataBase, InstanceDataBase, LocationDataBase, RingDataBase};
use driver::security::AuthorizeInMemoryInstance;
use driver::service::S3ImageStorageService;
use driver::{DataBaseInitializer, S3Credentials};
use kernel::finder::DependOnRingFinder;
use kernel::repository::{
    DependOnImageRepository, DependOnInstanceRepository, DependOnLocationRepository,
};
use kernel::security::DependOnAuthorizeAdminPolicy;
use kernel::service::DependOnImageExportExternalStorageService;
use std::sync::Arc;
use driver::volatile::LocationEtagVolatileDataBase;
use kernel::volatiles::DependOnLocationETagCache;

pub struct AppHandler {
    inner: Arc<Handler>,
}

impl AppHandler {
    pub async fn init() -> Result<AppHandler, ServerError> {
        Ok(Self {
            inner: Arc::new(Handler::init().await?),
        })
    }
}

impl Clone for AppHandler {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AsRef<Handler> for AppHandler {
    fn as_ref(&self) -> &Handler {
        &self.inner
    }
}

impl Deref for AppHandler {
    type Target = Handler;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct Handler {
    loc: LocationDataBase,
    ins: InstanceDataBase,
    img: ImageDataBase,
    ring: RingDataBase,
    auth: AuthorizeInMemoryInstance,

    cache_loc: LocationEtagVolatileDataBase,

    s3_images: S3ImageStorageService,
}

impl Handler {
    #[allow(dead_code)]
    async fn init() -> Result<Self, ServerError> {
        let pg_url = dotenvy::var("PG_DATABASE_URL")
            .map_err(|_| ServerError::EnvError(r#"PG_DATABASE_URL"#))?;

        let redis_url = dotenvy::var("REDIS_URL")
            .map_err(|_| ServerError::EnvError(r#"REDIS_URL"#))?;

        let anonymous = dotenvy::var("S3_ANONYMOUS")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);

        let creds = if anonymous {
            tracing::warn!("+ S3_ANONYMOUS is true. This is not secured.");
            S3Credentials::anonymous()
        } else {
            S3Credentials::from_env()
        }
        .map_err(|e| ServerError::Driver(anyhow::Error::new(e)))?;

        let bucket_name = dotenvy::var("S3_BUCKET_NAME")
            .map_err(|_| ServerError::EnvError(r#"S3_BUCKET_NAME"#))?;

        let bucket_region = dotenvy::var("S3_BUCKET_REGION")
            .map_err(|_| ServerError::EnvError(r#"S3_BUCKET_REGION"#))?;

        let one_time = kernel::entities::token::AdminToken::default();
        tracing::info!("+ Admin Token generated.");
        tracing::info!("| * {:?}", one_time);
        tracing::info!("| This token is available as long as this instance is active,");
        tracing::info!(
            "+ but is immediately discarded and a new token is regenerated when it is restarted."
        );

        let pg_pool = DataBaseInitializer::setup_postgres(pg_url).await?;
        let redis_pool = DataBaseInitializer::setup_redis(redis_url).await?;

        let use_localstack = dotenvy::var("S3_USE_LOCALSTACK")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);

        let s3_bucket = if use_localstack {
            DataBaseInitializer::setup_localstack(bucket_name, creds).await
        } else {
            DataBaseInitializer::setup_s3(bucket_name, bucket_region, creds).await
        }?;

        let loc = LocationDataBase::new(pg_pool.clone());
        let ins = InstanceDataBase::new(pg_pool.clone());
        let img = ImageDataBase::new(pg_pool.clone());
        let ring = RingDataBase::new(pg_pool);
        let auth = AuthorizeInMemoryInstance::new(one_time);

        let s3_images = S3ImageStorageService::new(s3_bucket);

        let cache_loc = LocationEtagVolatileDataBase::new(redis_pool);

        Ok(Self {
            loc,
            ins,
            img,
            ring,
            auth,
            s3_images,
            cache_loc
        })
    }
}

impl DependOnLocationRepository for Handler {
    type LocationRepository = LocationDataBase;
    fn location_repository(&self) -> &Self::LocationRepository {
        &self.loc
    }
}

impl DependOnLocationETagCache for Handler {
    type LocationETagCache = LocationEtagVolatileDataBase;
    fn location_e_tag_cache(&self) -> &Self::LocationETagCache {
        &self.cache_loc
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

impl DependOnCreateEmptyInstanceService for Handler {
    type CreateEmptyInstanceService = Self;
    fn create_empty_instance_service(&self) -> &Self::CreateEmptyInstanceService {
        self
    }
}

impl DependOnAuthorizeAdminPolicy for Handler {
    type AuthorizeAdminPolicy = AuthorizeInMemoryInstance;
    fn authorize_admin_policy(&self) -> &Self::AuthorizeAdminPolicy {
        &self.auth
    }
}

impl DependOnRingFinder for Handler {
    type RingFinder = RingDataBase;
    fn ring_finder(&self) -> &Self::RingFinder {
        &self.ring
    }
}

impl DependOnImageExportExternalStorageService for Handler {
    type ImageExportExternalStorageService = S3ImageStorageService;
    fn image_export_external_storage_service(&self) -> &Self::ImageExportExternalStorageService {
        &self.s3_images
    }
}

impl DependOnImageRepository for Handler {
    type ImageRepository = ImageDataBase;
    fn image_repository(&self) -> &Self::ImageRepository {
        &self.img
    }
}

impl DependOnExportImageService for Handler {
    type ExportImageService = Self;
    fn export_image_service(&self) -> &Self::ExportImageService {
        self
    }
}
