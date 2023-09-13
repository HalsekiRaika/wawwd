use async_trait::async_trait;
use geo_types::Geometry;
use geozero::wkb::Decode;
use sqlx::{PgConnection, Pool, Postgres, QueryBuilder};
use kernel::entities::geology::Position;
use kernel::entities::location::{LocalizedName, Location, LocationId};
use kernel::error::KernelError;
use kernel::external::uuid::Uuid;
use kernel::repository::LocationRepository;
use crate::database::BIND_LIMIT;
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
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        LocationDataBaseInternal::create(create, &mut con).await?;
        Ok(())
    }

    async fn update(&self, update: &Location) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        LocationDataBaseInternal::update(update, &mut con).await?;
        Ok(())
    }

    async fn delete(&self, delete: &LocationId) -> Result<(), KernelError> {
        todo!()
    }

    async fn find_by_id(&self, id: &LocationId) -> Result<Option<Location>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let loc = LocationDataBaseInternal::find_by_id(id, &mut con).await?;
        Ok(loc)
    }
}


#[allow(unused)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct LocationRow {
    pub id: Uuid,
    pub location: Decode<Geometry>
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct LocationLocalizedRow {
    pub id: Uuid,
    pub country: String,
    pub name: String
}


pub(in crate::database) struct LocationDataBaseInternal;


impl LocationDataBaseInternal {
    pub(in crate::database) async fn create(ctx: &Location, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO location_mark(
              id, location
            ) VALUES (
              $1, ST_SETSRID(ST_POINT($2, $3), 4326)
            )
        "#).bind(ctx.id().as_ref())
           .bind(ctx.pos().x().as_ref())
           .bind(ctx.pos().y().as_ref())
           .execute(&mut *con)
           .await?;

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"
            INSERT INTO location_mark_localized_name(
              id, country, name
            )
        "#);

        builder.push_values(ctx.localize().iter().take(BIND_LIMIT / 3), |mut b, loc| {
            b.push_bind(ctx.id().as_ref())
                .push_bind(loc.country())
                .push_bind(loc.localize());
        });

        builder.build()
            .execute(&mut *con)
            .await?;

        Ok(())
    }

    pub(in crate::database) async fn update(ctx: &Location, con: &mut PgConnection) -> Result<(), DriverError> {
        todo!()
    }

    pub(in crate::database) async fn find_by_id(id: &LocationId, con: &mut PgConnection) -> Result<Option<Location>, DriverError> {
        // language=SQL
        let mark = sqlx::query_as::<_, LocationRow>(r#"
            SELECT id, location::GEOMETRY FROM location_mark WHERE id = $1
        "#)
            .bind(id.as_ref())
            .fetch_optional(&mut *con)
            .await?;

        let Some(mark) = mark else {
            return Ok(None)
        };

        // language=SQL
        let localize = sqlx::query_as::<_, LocationLocalizedRow>(r#"
            SELECT * FROM location_mark_localized_name WHERE id = $1
        "#).bind(id.as_ref())
           .fetch_all(&mut *con)
           .await?;

        let lid = LocationId::new(mark.id);
        // Why `.unwrap()`?: Because `NOT NULL` is guaranteed by SQL constraints.
        let pos = mark.location.geometry.map(Position::try_from).transpose()?.unwrap();
        let loc = localize.into_iter()
            .map(|row| LocalizedName::new(row.country, row.name))
            .collect::<Vec<_>>();
        let loc = Location::new(lid, pos, loc);

        Ok(Some(loc))
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use sqlx::{Pool, Postgres};
    use sqlx::postgres::PgPoolOptions;
    use kernel::entities::geology::Position;
    use kernel::entities::location::{LocalizedName, Location, LocationId};
    use crate::database::location::LocationDataBaseInternal;

    async fn test_pool() -> anyhow::Result<Pool<Postgres>> {
        dotenvy::dotenv().ok();

        let url = dotenvy::var("PG_DATABASE_URL")
            .expect("`PG_DATABASE_URL` is not set. This is a required environment variable.");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .idle_timeout(Duration::new(5, 0))
            .connect(&url)
            .await?;

        Ok(pool)
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_insert() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let lid = LocationId::default();
        let pos = Position::new(132.76661710012877f64, 33.841405349477995f64)?;
        let loc = vec![
                ("jp", "愛媛県庁第一別館"),
                ("en", "Ehime Prefectural Office Branch Office")
            ]
            .into_iter()
            .map(|(c, n)| LocalizedName::new(c, n))
            .collect::<Vec<_>>();
        let loc = Location::new(lid, pos, loc);

        LocationDataBaseInternal::create(&loc, &mut transaction).await?;
        let loc = LocationDataBaseInternal::find_by_id(&lid, &mut transaction).await?;

        println!("{:#?}", loc);

        transaction.rollback().await?;

        Ok(())
    }
}