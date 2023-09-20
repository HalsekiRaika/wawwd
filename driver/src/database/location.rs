use crate::database::BIND_LIMIT;
use crate::error::DriverError;
use async_trait::async_trait;
use geo_types::Geometry;
use geozero::wkb::Decode;
use kernel::entities::geology::Position;
use kernel::entities::location::{LocalizeId, LocalizedName, Location, LocationId};
use kernel::error::KernelError;
use kernel::external::uuid::Uuid;
use kernel::repository::LocationRepository;
use sqlx::{PgConnection, Pool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct LocationDataBase {
    pool: Pool<Postgres>,
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
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        LocationDataBaseInternal::delete(delete, &mut con).await?;
        Ok(())
    }

    async fn delete_localize(
        &self,
        delete: &LocationId,
        code: &LocalizeId,
    ) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        LocationDataBaseInternal::delete_localize(delete, code, &mut con).await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Location>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let loc = LocationDataBaseInternal::find_all(&mut con).await?;
        Ok(loc)
    }

    async fn find_by_id(&self, id: &LocationId) -> Result<Option<Location>, KernelError> {
        let mut con = self.pool.acquire().await.map_err(DriverError::from)?;
        let loc = LocationDataBaseInternal::find_by_id(id, &mut con).await?;
        Ok(loc)
    }
}


#[allow(unused)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct LocationMarkRow {
    pub id: Uuid,
    pub location: Decode<Geometry>,
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct LocationLocalizedRow {
    pub id: Uuid,
    pub country: String,
    pub name: String,
}

pub(in crate::database) struct LocationDataBaseInternal;

impl LocationDataBaseInternal {
    // noinspection DuplicatedCode
    pub(in crate::database) async fn create(
        ctx: &Location,
        con: &mut PgConnection,
    ) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(
            r#"
            INSERT INTO location_mark(
              id, location
            ) VALUES (
              $1, ST_SETSRID(ST_POINT($2, $3), 4326)
            )
        "#,
        )
        .bind(ctx.id().as_ref())
        .bind(ctx.pos().x().as_ref())
        .bind(ctx.pos().y().as_ref())
        .execute(&mut *con)
        .await?;

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO location_mark_localized_name(
              id, country, name
            )
        "#,
        );

        builder.push_values(ctx.localize().iter().take(BIND_LIMIT / 3), |mut b, loc| {
            b.push_bind(ctx.id().as_ref())
                .push_bind(loc.country().as_ref())
                .push_bind(loc.localize());
        });

        builder.build().execute(&mut *con).await?;

        Ok(())
    }

    // noinspection DuplicatedCode
    pub(in crate::database) async fn update(
        ctx: &Location,
        con: &mut PgConnection,
    ) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(
            r#"
            UPDATE location_mark
              SET location = ST_SETSRID(ST_POINT($1, $2), 4326)
            WHERE id = $3
        "#,
        )
        .bind(ctx.pos().x().as_ref())
        .bind(ctx.pos().y().as_ref())
        .bind(ctx.id().as_ref())
        .execute(&mut *con)
        .await?;

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO location_mark_localized_name (
              id, country, name
            )
        "#,
        );

        query.push_values(ctx.localize().iter().take(BIND_LIMIT / 3), |mut b, loc| {
            b.push_bind(ctx.id().as_ref())
                .push_bind(loc.country().as_ref())
                .push_bind(loc.localize());
        });

        query.push(
            r#"
            ON CONFLICT(id, country) DO UPDATE
              SET
                name = EXCLUDED.name
        "#,
        );

        query.build().execute(&mut *con).await?;

        Ok(())
    }

    pub(in crate::database) async fn delete(
        id: &LocationId,
        con: &mut PgConnection,
    ) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(
            r#"
          DELETE FROM location_mark WHERE id = $1
        "#,
        )
        .bind(id.as_ref())
        .execute(&mut *con)
        .await?;
        Ok(())
    }

    pub(in crate::database) async fn delete_localize(
        id: &LocationId,
        code: &LocalizeId,
        con: &mut PgConnection,
    ) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(
            r#"
          DELETE FROM location_mark_localized_name WHERE id = $1 AND country LIKE $2
        "#,
        )
        .bind(id.as_ref())
        .bind(code.as_ref())
        .execute(&mut *con)
        .await?;
        Ok(())
    }

    pub(in crate::database) async fn find_all(con: &mut PgConnection) -> Result<Vec<Location>, DriverError> {
        // language=SQL
        let mark = sqlx::query_as::<_, LocationMarkRow>(r#"
            SELECT id, location::GEOMETRY FROM location_mark
        "#).fetch_all(&mut *con)
            .await?;

        // language=SQL
        let localize = sqlx::query_as::<_, LocationLocalizedRow>(r#"
        "#).fetch_all(&mut *con)
            .await?;

        let loc = mark.into_iter()
            .map(|mark| {
                let loc = localize.iter()
                    .filter(|loc| loc.id.eq(&mark.id))
                    .map(|f| LocalizedName::new(&f.country, &f.name))
                    .collect::<Result<Vec<_>, _>>()?;
                Location::r#try(mark.id, mark.location.geometry.unwrap(), loc)
            })
            .collect::<Result<Vec<Location>, _>>()?;

        Ok(loc)
    }

    pub(in crate::database) async fn find_by_id(
        id: &LocationId,
        con: &mut PgConnection,
    ) -> Result<Option<Location>, DriverError> {
        // language=SQL
        let mark = sqlx::query_as::<_, LocationMarkRow>(
            r#"
            SELECT id, location::GEOMETRY FROM location_mark WHERE id = $1
        "#,
        )
        .bind(id.as_ref())
        .fetch_optional(&mut *con)
        .await?;

        let Some(mark) = mark else { return Ok(None) };

        // language=SQL
        let localize = sqlx::query_as::<_, LocationLocalizedRow>(
            r#"
            SELECT * FROM location_mark_localized_name WHERE id = $1
        "#,
        )
        .bind(id.as_ref())
        .fetch_all(&mut *con)
        .await?;

        let lid = LocationId::new(mark.id);
        // Why `.unwrap()`?: Because `NOT NULL` is guaranteed by SQL constraints.
        let pos = mark
            .location
            .geometry
            .map(Position::try_from)
            .transpose()?
            .unwrap();
        let loc = localize
            .into_iter()
            .map(|row| LocalizedName::new(row.country, row.name))
            .collect::<Result<Vec<LocalizedName>, _>>()?;
        let loc = Location::new(lid, pos, loc);

        Ok(Some(loc))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::location::LocationDataBaseInternal;
    use kernel::entities::geology::Position;
    use kernel::entities::location::{LocalizeId, LocalizedName, Location, LocationId};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{PgConnection, Pool, Postgres};
    use std::time::Duration;

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

    async fn create(con: &mut PgConnection) -> anyhow::Result<Location> {
        let lid = LocationId::default();
        let pos = Position::new(132.76661710012877f64, 33.841405349477995f64)?;
        let loc = vec![
            ("jp", "愛媛県庁第一別館"),
            ("en", "Ehime Prefectural Office Branch Office"),
        ]
        .into_iter()
        .map(|(c, n)| LocalizedName::new(c, n))
        .collect::<Result<Vec<_>, _>>()?;
        let loc = Location::new(lid, pos, loc);

        LocationDataBaseInternal::create(&loc, &mut *con).await?;
        let loc = LocationDataBaseInternal::find_by_id(&lid, &mut *con).await?;

        loc.ok_or(anyhow::Error::msg("Failed insert data."))
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_insert() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let loc = create(&mut transaction).await?;
        println!("{:#?}", loc);

        transaction.rollback().await?;

        Ok(())
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_update() -> anyhow::Result<()> {
        let pool = test_pool().await?;
        let mut transaction = pool.begin().await?;

        let loc = create(&mut transaction).await?;
        println!("{:#?}", loc);

        let mut location = loc.into_destruct();

        let pos = Position::new(131.76661710012877f64, 32.841405349477995f64)?;
        let loc = vec![
            ("jp", "愛媛県庁第１別館"),
            ("en", "Ehime Prefecture Office Branch Office"),
        ]
        .into_iter()
        .map(|(c, n)| LocalizedName::new(c, n))
        .collect::<Result<Vec<_>, _>>()?;

        location.pos = pos;
        location.localize = loc;

        let loc = location.freeze();

        LocationDataBaseInternal::update(&loc, &mut transaction).await?;
        let upd = LocationDataBaseInternal::find_by_id(loc.id(), &mut transaction).await?;

        println!("{:#?}", upd);

        transaction.rollback().await?;

        Ok(())
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_delete() -> anyhow::Result<()> {
        let pool = test_pool().await?;
        let mut transaction = pool.begin().await?;

        let loc = create(&mut transaction).await?;
        println!("{:?}", loc);

        let loc = LocationDataBaseInternal::find_by_id(loc.id(), &mut transaction).await?;
        let loc = loc.ok_or(anyhow::Error::msg("cannot find `location`."))?;
        println!("{:?}", loc);

        let lci = LocalizeId::new("en")?;
        LocationDataBaseInternal::delete_localize(loc.id(), &lci, &mut transaction).await?;

        let loc = LocationDataBaseInternal::find_by_id(loc.id(), &mut transaction).await?;
        let loc = loc.ok_or(anyhow::Error::msg("cannot find `location`."))?;
        println!("{:?}", loc);

        LocationDataBaseInternal::delete(loc.id(), &mut transaction).await?;

        let loc = LocationDataBaseInternal::find_by_id(loc.id(), &mut transaction).await?;
        println!("{:?}", loc);

        Ok(())
    }
}
