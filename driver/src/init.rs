use crate::error::DriverError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub struct DataBaseInitializer;

impl DataBaseInitializer {
    pub async fn setup(url: impl AsRef<str>) -> Result<Pool<Postgres>, DriverError> {
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
}
