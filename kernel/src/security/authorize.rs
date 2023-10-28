use crate::entities::token::AdminToken;
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait AuthorizeAdminPolicy: 'static + Sync + Send {
    async fn authorize(&self, token: &AdminToken) -> Result<(), KernelError>;
}
