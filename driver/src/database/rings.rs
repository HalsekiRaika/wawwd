use crate::database::instance::RingRow;
use crate::error::DriverError;
use async_trait::async_trait;
use kernel::entities::ring::{Ring, RingId};
use kernel::error::KernelError;
use kernel::finder::RingFinder;
use sqlx::{PgConnection, Pool, Postgres};

pub struct RingDataBase {
    pool: Pool<Postgres>,
}

impl RingDataBase {
    pub fn new(pool: Pool<Postgres>) -> RingDataBase {
        Self { pool }
    }
}

#[async_trait]
impl RingFinder for RingDataBase {
    async fn find_by_id(&self, id: &RingId) -> Result<Option<Ring>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let found = RingDataBaseInternalProcessor::find_by_id(id, &mut con).await?;
        Ok(found)
    }
}

pub struct RingDataBaseInternalProcessor;

impl RingDataBaseInternalProcessor {
    #[rustfmt::skip]
    pub async fn find_by_id(id: &RingId, con: &mut PgConnection) -> Result<Option<Ring>, DriverError> {
        // language=SQL
        let row = sqlx::query_as::<_, RingRow>(r#"
            SELECT
                id, instance, pos_in::GEOMETRY, hue, user_id, index, created_at
            FROM
                rings
            WHERE
                id = $1
        "#)
            .bind(id.as_ref())
            .fetch_optional(con)
            .await?;

        row.map(TryInto::try_into).transpose()
    }
}
