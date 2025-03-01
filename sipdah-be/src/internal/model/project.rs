// use std::pin::Pin;
// use chrono::{DateTime, Utc};
// use tokio::fs::File;
// use tokio::io::{AsyncRead, AsyncWriteExt, BufWriter};
// use uuid::Uuid;
// 
// #[derive(Debug)]
// pub enum Status {
//     GatherRequirement,
//     Acknowledge,
//     Design,
//     Develop,
//     Test,
//     Deliver,
//     Complete,
// }
// 
// #[derive(Debug)]
// pub struct Project {
//     pub id: Uuid,
//     pub author_id: Uuid,
//     pub name: String,
//     pub status: Status,
//     pub logo_url: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
//     pub deleted_at: Option<DateTime<Utc>>,
// }
// 
// pub trait Repository {
//     async fn create(&self);
// }
// 
// pub struct CreateProjectRequest {
//     pub name: String,
//     pub logo: Option<Pin<Box<dyn AsyncRead + Send>>>,
// }
// 
// async fn create(request: CreateProjectRequest) -> String {
//   
//     let created_at = Utc::now();
// 
//     // Simpan file ke disk jika ada
//     if let Some(mut logo_stream) = request.logo {
//         let file_path = format!("uploads/{}.png", "project_id");
//         let file = File::create(&file_path).await.unwrap();
//         let mut writer = BufWriter::new(file);
// 
//         tokio::io::copy(&mut logo_stream, &mut writer).await.unwrap();
//         writer.flush().await.unwrap();
//     }
// 
//     "".to_string()
// }