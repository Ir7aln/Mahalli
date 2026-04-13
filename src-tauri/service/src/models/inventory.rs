use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInventory {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub price: f64,
    pub quantity: f64,
    pub transaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInventory {
    pub transaction_type: String,
    pub product_id: String,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Inventory {
    pub id: String,
    pub transaction_type: String,
    pub product_id: String,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InventoryResponse {
    pub count: u64,
    pub inventory: Vec<SelectInventory>,
}
