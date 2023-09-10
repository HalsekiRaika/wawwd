use async_trait::async_trait;
use sqlx::{PgConnection, Pool, Postgres};
use geozero::wkt::
use kernel::entities::location::{Location, LocationId};
use kernel::error::KernelError;
use kernel::repository::LocationRepository;
use crate::error::DriverError;

#[derive(Clone)]
pub struct LocationDataBase {
    pool: Pool<Postgres>
}

impl LocationDataBase {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl LocationRepository for LocationDataBase {
    async fn create(&self, create: &Location) -> Result<(), KernelError> {
        todo!()
    }

    async fn update(&self, update: &Location) -> Result<(), KernelError> {
        todo!()
    }

    async fn delete(&self, delete: &LocationId) -> Result<(), KernelError> {
        todo!()
    }

    async fn find_by_id(&self, id: &LocationId) -> Result<Option<Location>, KernelError> {
        todo!()
    }
}


pub(in crate::database) struct LocationDataBaseInternal;

impl LocationDataBaseInternal {
    pub(in crate::database) async fn create(ctx: &Location, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO location_mark(id, location) VALUES ($1, ST_GEOGFROMTEXT('SRID=4326;'))
        "#).bind(ctx.id().as_ref())
            .bind(ctx.pos())
            .execute(&mut *con)
            .await?;
        todo!()
    }
}