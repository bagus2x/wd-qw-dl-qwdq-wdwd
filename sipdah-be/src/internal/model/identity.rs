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
        .map_err(|_| Error::Unauthorized("Not authorized".to_string()))
}
