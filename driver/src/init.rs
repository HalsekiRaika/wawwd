use crate::error::DriverError;
use s3::{Bucket, Region};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use deadpool_redis::{Config, Pool as RedisPool, Runtime};

pub use s3::creds::Credentials as S3Credentials;

pub struct DataBaseInitializer;

impl DataBaseInitializer {
    pub async fn setup_postgres(url: impl AsRef<str>) -> Result<Pool<Postgres>, DriverError> {
        let pg_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_millis(5000))
            .max_connections(8)
            .connect(url.as_ref())
            .await
            .map_err(|e| DriverError::DataBaseInitialization(anyhow::Error::new(e)))?;

        sqlx::migrate!("../migrations")
            .run(&pg_pool)
            .await
            .map_err(|e| DriverError::DataBaseInitialization(anyhow::Error::new(e)))?;

        Ok(pg_pool)
    }

    pub async fn setup_redis(url: impl AsRef<str>) -> Result<RedisPool, DriverError> {
        let redis_pool = Config::from_url(url.as_ref())
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|e| DriverError::DataBaseInitialization(anyhow::Error::new(e)))?;

        Ok(redis_pool)
    }

    pub async fn setup_s3(
        bucket_name: impl AsRef<str>,
        bucket_region: impl AsRef<str>,
        credentials: S3Credentials,
    ) -> Result<Bucket, DriverError> {
        let bucket = Bucket::new(
            bucket_name.as_ref(),
            bucket_region
                .as_ref()
                .parse()
                .map_err(|e| DriverError::S3(anyhow::Error::new(e)))?,
            credentials,
        )?;
        Ok(bucket)
    }

    pub async fn setup_localstack(
        bucket_name: impl AsRef<str>,
        credentials: S3Credentials,
    ) -> Result<Bucket, DriverError> {
        let mut bucket = Bucket::new(
            bucket_name.as_ref(),
            Region::Custom {
                region: "us-east-1".to_string(),
                endpoint: "http://localhost:4566".to_string(),
            },
            credentials,
        )?;

        bucket.set_path_style();

        Ok(bucket)
    }
}
