use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct DatabaseRecord {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub file_name: String,
    pub file_path: String,
    pub is_active: bool,
    pub created_from_database_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_opened_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct CreateDatabaseInput {
    pub name: String,
    pub slug: String,
    pub file_name: String,
    pub file_path: String,
    pub created_from_database_id: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct ActivateDatabaseInput {
    pub id: String,
}
