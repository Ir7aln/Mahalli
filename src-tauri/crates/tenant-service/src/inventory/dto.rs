use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListInventoryArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub transaction_type: Option<String>,
    pub created_from: Option<String>,
    pub created_to: Option<String>,
    #[specta(type = Option<f32>)]
    pub quantity_min: Option<f32>,
    #[specta(type = Option<f32>)]
    pub quantity_max: Option<f32>,
    #[specta(type = Option<f32>)]
    pub price_min: Option<f32>,
    #[specta(type = Option<f32>)]
    pub price_max: Option<f32>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInventory {
    pub id: String,
    pub name: String,
    pub created_at: String,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
    pub transaction_type: String,
    pub order_id: Option<String>,
    pub order_identifier: Option<String>,
    pub invoice_id: Option<String>,
    pub invoice_identifier: Option<String>,
    pub quote_id: Option<String>,
    pub quote_identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInventory {
    pub transaction_type: String,
    pub product_id: String,
    #[specta(type = f32)]
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Inventory {
    pub id: String,
    pub transaction_type: String,
    pub product_id: String,
    #[specta(type = f32)]
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InventoryResponse {
    pub count: u64,
    pub inventory: Vec<SelectInventory>,
}
