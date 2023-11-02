use uuid::Uuid;

pub type UniqId = String;

/// Генерирует уникальный случайный ID
pub fn gen_id() -> UniqId {
    Uuid::new_v4().to_string()
}
