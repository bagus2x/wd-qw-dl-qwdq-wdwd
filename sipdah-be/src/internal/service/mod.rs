use tokio::task_local;

pub mod auth;
pub mod user;

task_local! {
    pub static USER_ID: (String, String) // (id, email)
}