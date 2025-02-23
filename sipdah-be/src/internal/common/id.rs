use uuid::Uuid;

pub fn new() -> String {
    Uuid::now_v7().to_string()
}