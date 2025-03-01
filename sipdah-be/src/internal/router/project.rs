// use axum::{
//     extract::Multipart,
//     response::{Html, IntoResponse}
//     ,
// };
// use tokio::io::AsyncWriteExt;
// use tokio::{fs::File, io::BufWriter};
//
// async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
//     while let Some(mut field) = multipart.next_field().await.unwrap() {
//         if let Some(file_name) = field.file_name().map(String::from) {
//             let file_path = format!("uploads/{}", file_name);
//             let file = File::create(&file_path).await.unwrap();
//             let mut writer = BufWriter::new(file);
//
//             // Stream data in chunks and write to file
//             while let Some(chunk) = field.chunk().await.unwrap() {
//                 writer.write_all(&chunk).await.unwrap();
//             }
//
//             writer.flush().await.unwrap();
//             println!("Uploaded: {}", file_path);
//         }
//     }
//
//     Html("<h3>File uploaded successfully!</h3>")
// }
