use crate::error::DriverError;
use async_trait::async_trait;
use kernel::entities::image::{Image, ImageBin};
use kernel::entities::ring::{CreatedAt, RingId};
use kernel::error::KernelError;
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use kernel::repository::ImageRepository;
use sqlx::{PgConnection, Pool, Postgres};

#[allow(dead_code)]
pub struct ImageDataBase {
    pool: Pool<Postgres>,
}

impl ImageDataBase {
    pub fn new(pool: Pool<Postgres>) -> ImageDataBase {
        Self { pool }
    }
}

#[async_trait]
impl ImageRepository for ImageDataBase {
    async fn create(&self, create: &Image) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        ImageDataBaseInternalProcessor::create(create, &mut con).await?;
        Ok(())
    }

    async fn delete(&self, id: &RingId) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        ImageDataBaseInternalProcessor::delete(id, &mut con).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &RingId) -> Result<Option<Image>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let found = ImageDataBaseInternalProcessor::find_by_id(id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct ImageDataBaseInternalProcessor;

#[derive(sqlx::FromRow)]
pub(in crate::database) struct ImageRow {
    id: Uuid,
    image: Vec<u8>,
    created_at: OffsetDateTime,
}

impl From<ImageRow> for Image {
    fn from(value: ImageRow) -> Self {
        Self::new(
            RingId::new(value.id),
            ImageBin::new(value.image),
            CreatedAt::new(value.created_at),
        )
    }
}

impl ImageDataBaseInternalProcessor {
    #[rustfmt::skip]
    pub async fn create(create: &Image, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO images (id, image, created_at)
              VALUES ($1, $2, $3)
        "#)
            .bind(create.id().as_ref())
            .bind(create.bin().as_ref())
            .bind(create.created_at().as_ref())
            .execute(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn delete(id: &RingId, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            DELETE FROM images WHERE id = $1
        "#)
            .bind(id.as_ref())
            .execute(&mut *con)
            .await?;
        Ok(())
    }

    #[rustfmt::skip]
    pub async fn find_by_id(id: &RingId, con: &mut PgConnection) -> Result<Option<Image>, DriverError> {
        // language=SQL
        let row = sqlx::query_as::<_, ImageRow>(r#"
            SELECT id, image, created_at
              FROM images
             WHERE id = $1
        "#)
            .bind(id.as_ref())
            .fetch_optional(&mut *con)
            .await?;
        Ok(row.map(Into::into))
    }
}
