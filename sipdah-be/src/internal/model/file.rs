// use crate::internal::model::error::Error;
// use chrono::{DateTime, Local};
// 
// pub struct File {
//     pub id: String,
//     pub file_name: String,
//     pub file_size: u64,
//     pub file_path: u64,
//     pub file_type: String,
//     pub location: String,
//     pub owner_id: Option<String>,
//     pub category: String,
//     pub created_at: DateTime<Local>,
//     pub updated_at: DateTime<Local>,
//     pub deleted_at: Option<DateTime<Local>>,
// }
// 
// pub trait FileRepository {
//     async fn save(file: File) -> Result<(), Error>;
// 
//     async fn find_by_id(file: File) -> Result<(), Error>;
// }
