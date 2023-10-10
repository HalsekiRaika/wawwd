mod create;
mod delete;
mod update;

pub use self::{create::*, delete::DeleteRequestToDeleteLocationDto, update::*};

pub mod form {
    pub use super::delete::DeleteRequest;
}
