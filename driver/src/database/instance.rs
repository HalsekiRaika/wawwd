use std::collections::BTreeSet;
use std::net::IpAddr;
use async_trait::async_trait;
use geo_types::Geometry;
use geozero::wkb::Decode;
use kernel::repository::InstanceRepository;
use sqlx::{PgConnection, Pool, Postgres, QueryBuilder};
use kernel::entities::geology::Position;
use kernel::entities::instance::{FinishedAt, Instance, InstanceId, RingSet, StartedAt};
use kernel::entities::location::LocationId;
use kernel::entities::ring::{CreatedAt, HueColor, Index, Ring, RingId, UserIp};
use kernel::error::KernelError;
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;
use crate::database::BIND_LIMIT;
use crate::error::DriverError;

#[derive(Clone)]
pub struct InstanceDataBase {
    pool: Pool<Postgres>,
}

impl InstanceDataBase {
    pub fn new(pool: Pool<Postgres>) -> InstanceDataBase {
        Self { pool }
    }
}

#[async_trait]
impl InstanceRepository for InstanceDataBase {
    async fn create(&self, create: &Instance) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        InternalInstanceDataBase::create(create, &mut con).await?;
        Ok(())
    }
    async fn update(&self, update: &Instance) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        InternalInstanceDataBase::update(update, &mut con).await?;
        Ok(())
    }
    async fn delete(&self, delete: &InstanceId) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        InternalInstanceDataBase::delete(delete, &mut con).await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<BTreeSet<Instance>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let all = InternalInstanceDataBase::find_all(&mut con).await?;
        Ok(all)
    }

    async fn find_by_id(&self, id: &InstanceId) -> Result<Option<Instance>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let found = InternalInstanceDataBase::find_by_id(id, &mut con).await?;
        Ok(found)
    }

    async fn find_unfinished(&self) -> Result<Option<Instance>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let found = InternalInstanceDataBase::find_unfinished(&mut con).await?;
        Ok(found)
    }
}


#[derive(sqlx::FromRow)]
pub(in crate::database) struct InstanceRow {
    id: Uuid,
    location: Uuid,
    started_at: OffsetDateTime,
    finished_at: Option<OffsetDateTime>,
}

impl TryFrom<InstanceRow> for Instance {
    type Error = KernelError;
    fn try_from(value: InstanceRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            InstanceId::new(value.id),
            LocationId::new(value.location),
            RingSet::default(),
            StartedAt::new(value.started_at),
            FinishedAt::new::<OffsetDateTime>(value.finished_at)
        ))
    }
}

#[derive(sqlx::FromRow)]
pub(in crate::database) struct RingRow {
    id: Uuid,
    pos_in: Decode<Geometry>,
    hue: i32,
    addr: IpAddr,
    index: i32,
    created_at: OffsetDateTime
}

impl TryFrom<RingRow> for Ring {
    type Error = DriverError;
    fn try_from(value: RingRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            RingId::new(value.id),
            Position::try_from(value.pos_in.geometry.ok_or(DriverError::Decoding {column: "pos_in"})?)?,
            UserIp::try_from(value.addr)?,
            Index::new(value.index)?,
            HueColor::new(value.hue),
            CreatedAt::new(value.created_at)
        ))
    }
}

pub(in crate::database) struct InternalInstanceDataBase;

impl InternalInstanceDataBase {
    #[rustfmt::skip]
    pub(in crate::database) async fn create(create: &Instance, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO instances(
              id, location, started_at, finished_at
            ) VALUES (
              $1, $2, $3, $4
            )
        "#)
            .bind(create.id().as_ref())
            .bind(create.location().as_ref())
            .bind(create.started_at().as_ref())
            .bind(create.finished_at().as_ref())
            .execute(&mut *con)
            .await?;
        Ok(())
    }

    #[rustfmt::skip]
    pub(in crate::database) async fn update(update: &Instance, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            UPDATE instances
              SET
                finished_at = $1
            WHERE
              id = $2
        "#)
            .bind(update.finished_at().as_ref())
            .bind(update.id().as_ref())
            .execute(&mut *con)
            .await?;


        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(r#"
            INSERT INTO rings (
              id, instance, pos_in, hue, addr, index, created_at
            )
        "#);

        query.push_values(update.rings().iter().take(BIND_LIMIT / 7), |mut b, ring| {
            b.push_bind(ring.id().as_ref())
                .push_bind(update.id().as_ref())
                .push(r#"ST_SETSRID(ST_POINT("#)
                .push_bind_unseparated(ring.pos_in().x().as_ref())
                .push_bind(ring.pos_in().y().as_ref())
                .push_unseparated(r#"), 4326)"#)
                .push_bind(ring.color().as_ref())
                .push_bind(ring.addr().as_ref())
                .push_bind(ring.index().as_ref())
                .push_bind(ring.created_at().as_ref());
        });

        query.push("ON CONFLICT(id) DO NOTHING");

        query.build().execute(&mut *con).await?;

        Ok(())
    }

    #[rustfmt::skip]
    pub(in crate::database) async fn delete(delete: &InstanceId, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            DELETE FROM instances WHERE id = $1
        "#)
            .bind(delete.as_ref())
            .execute(&mut *con)
            .await?;
        Ok(())
    }

    #[rustfmt::skip]
    pub(in crate::database) async fn find_all(con: &mut PgConnection) -> Result<BTreeSet<Instance>, DriverError> {
        // language=SQL
        let all = sqlx::query_as::<_, InstanceRow>(r#"
            SELECT id, location, started_at, finished_at FROM instances
        "#)
            .fetch_all(&mut *con)
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<BTreeSet<Instance>, KernelError>>()?;

        Ok(all)
    }

    #[rustfmt::skip]
    pub(in crate::database) async fn find_by_id(id: &InstanceId, con: &mut PgConnection) -> Result<Option<Instance>, DriverError> {
        // language=SQL
        let i_row = sqlx::query_as::<_, InstanceRow>(r#"
            SELECT id, location, started_at, finished_at FROM instances WHERE id = $1
        "#)
            .bind(id.as_ref())
            .fetch_optional(&mut *con)
            .await?
            .map(TryInto::try_into)
            .transpose()?;

        // language=SQL
        let r_row = sqlx::query_as::<_, RingRow>(r#"
            SELECT id, pos_in::GEOMETRY, hue, addr, index, created_at FROM rings WHERE instance = $1
        "#)
            .bind(id.as_ref())
            .fetch_all(&mut *con)
            .await?
            .into_iter()
            .map(|row| -> Result<Ring, KernelError> {
                let id = RingId::new(row.id);
                let pos_in = Position::try_from(row.pos_in.geometry.unwrap())?;
                let addr = UserIp::new(row.addr.to_string())?;
                let index = Index::new(row.index)?;
                let color = HueColor::new(row.hue);
                let created_at = CreatedAt::new(row.created_at);
                Ok(Ring::new(id, pos_in, addr, index, color, created_at))
            })
            .collect::<Result<Vec<_>, KernelError>>()?;

        let found = i_row.map(|instance: Instance| instance.into_destruct())
            .map(|mut des| -> Result<Instance, KernelError> {
                let rings = RingSet::new(r_row)?;
                des.rings = rings;
                Ok(des.freeze())
            })
            .transpose()?;

        Ok(found)
    }

    #[rustfmt::skip]
    pub(in crate::database) async fn find_unfinished(con: &mut PgConnection) -> Result<Option<Instance>, DriverError> {
        // language=SQL
        let instance = sqlx::query_as::<_, InstanceRow>(r#"
            SELECT id, location, started_at, finished_at FROM instances WHERE instances.finished_at IS NULL
        "#)
            .fetch_optional(&mut *con)
            .await?;

        let rings = if let Some(instance) = &instance {
            // language=SQL
            sqlx::query_as::<_, RingRow>(r#"
                SELECT id, instance, pos_in::GEOMETRY, hue, addr, index, created_at FROM rings WHERE instance = $1
            "#)
                .bind(instance.id)
                .fetch_all(&mut *con)
                .await
        } else {
            return Ok(None)
        }?;

        let rings = rings.into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<Ring>, DriverError>>()?;

        let found = instance.map(TryInto::try_into)
            .transpose()?
            .map(|ins: Instance| ins.into_destruct())
            .map(|mut ins| -> Result<Instance, KernelError> {
                ins.rings = RingSet::new(rings)?;
                Ok(ins.freeze())
            })
            .transpose()?;

        Ok(found)
    }
}