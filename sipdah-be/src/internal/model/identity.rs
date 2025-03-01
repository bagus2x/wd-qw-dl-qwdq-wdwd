use crate::internal::model::error::Error;
use tokio::task_local;

#[derive(Clone)]
pub struct Identity {
    pub user_id: String,
    pub email: String,
}

task_local! {
    pub static IDENTITY: Identity
}

pub fn get_current_identity() -> Result<Identity, Error> {
    IDENTITY
        .try_with(|identity| identity.clone())
        // Authorization must be handle in router
        .map_err(|_| Error::Forbidden("Failed to retrieve current identity".to_string()))
}
