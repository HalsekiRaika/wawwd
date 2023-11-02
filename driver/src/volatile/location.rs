use async_trait::async_trait;
use deadpool_redis::{redis, Pool, Connection as RedisConnection};
use kernel::entities::volatiles::Etag;
use kernel::error::KernelError;
use kernel::volatiles::LocationETagCache;
use crate::error::DriverError;

pub struct LocationEtagVolatileDataBase {
    pool: Pool,
}

impl LocationEtagVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LocationETagCache for LocationEtagVolatileDataBase {
    async fn save(&self, tag: Etag) -> Result<(), KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        Internal::save(Self::NAMESPACE, tag, &mut con).await.map_err(DriverError::from)?;
        Ok(())
    }

    async fn dele(&self) -> Result<(), KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        Internal::dele(Self::NAMESPACE, &mut con).await.map_err(DriverError::from)?;
        Ok(())
    }

    async fn find(&self) -> Result<Option<Etag>, KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        let found = Internal::find(Self::NAMESPACE, &mut con).await.map_err(DriverError::from)?;
        Ok(found)
    }
}


pub(in crate) struct Internal;

impl Internal {
    pub async fn save(key: &str, tag: Etag, con: &mut RedisConnection) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(key)
            .arg(tag.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn dele(key: &str, con: &mut RedisConnection) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(key: &str, con: &mut RedisConnection) -> Result<Option<Etag>, DriverError> {
        let tag: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut *con)
            .await?;
        Ok(tag.map(Etag::unchecked_new))
    }
}
