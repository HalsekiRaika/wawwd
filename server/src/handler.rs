use crate::error::ServerError;

#[derive(Clone)]
pub struct Handler {

}

impl Handler {
    #[allow(dead_code)]
    pub async fn init() -> Result<Self, ServerError> {
        unimplemented!()
    }
}