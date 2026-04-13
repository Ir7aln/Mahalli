use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Type)]
pub struct UpdateStatus {
    pub id: String,
    pub status: String,
}
