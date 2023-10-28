use async_trait::async_trait;
use kernel::entities::token::AdminToken;
use kernel::error::KernelError;
use kernel::security::AuthorizeAdminPolicy;

#[derive(Clone)]
pub struct AuthorizeInMemoryInstance {
    token: AdminToken,
}

impl AuthorizeInMemoryInstance {
    pub fn new(token: AdminToken) -> AuthorizeInMemoryInstance {
        Self { token }
    }
}

#[async_trait]
impl AuthorizeAdminPolicy for AuthorizeInMemoryInstance {
    async fn authorize(&self, token: &AdminToken) -> Result<(), KernelError> {
        if self.token.eq(token) {
            return Ok(());
        }
        Err(KernelError::Validation {
            msg: "Invalid authorize token.",
        })
    }
}
