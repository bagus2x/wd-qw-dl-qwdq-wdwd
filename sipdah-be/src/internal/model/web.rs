use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageRequest {
    pub cursor: Option<String>,
    pub size: Option<u32>,
}

// impl PageRequest {
//     pub fn size(&self) -> u32 {
//         if let Some(size) = self.size { size } else { 10 }
//     }
//
//     pub fn cursor(&self) -> String {
//         if let Some(cursor) = &self.cursor {
//             cursor.clone()
//         } else {
//             id::new()
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<i64>,
    pub size: i32,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: u16,
    pub message: String,
}
