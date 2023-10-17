use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use kernel::repository::InstanceRepository;

pub struct InstanceDataBase {
    pool: Pool<Postgres>
}

impl InstanceDataBase {
    pub fn new(pool: Pool<Postgres>) -> InstanceDataBase {
        Self { pool }
    }
}

// #[async_trait]
// impl InstanceRepository for InstanceDataBase {
//     async fn create
// }