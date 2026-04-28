use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListInventoryArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub transaction_type: Option<String>,
    pub source_type: Option<String>,
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
    pub include_voided: Option<bool>,
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
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_identifier: Option<String>,
    pub notes: Option<String>,
    pub is_void: bool,
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
    pub source_type: Option<String>,
    pub source_id: Option<String>,
    #[specta(type = Option<f32>)]
    pub unit_price: Option<f32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct VoidInventoryArgs {
    pub id: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InventoryResponse {
    pub count: u64,
    pub inventory: Vec<SelectInventory>,
}
