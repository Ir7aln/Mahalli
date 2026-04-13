use sea_orm::entity::prelude::Decimal;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInventory {
    pub id: String,
    pub name: String,
    pub created_at: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub transaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInventory {
    pub transaction_type: String,
    pub product_id: String,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Inventory {
    pub id: String,
    pub transaction_type: String,
    pub product_id: String,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InventoryResponse {
    pub count: u64,
    pub inventory: Vec<SelectInventory>,
}
