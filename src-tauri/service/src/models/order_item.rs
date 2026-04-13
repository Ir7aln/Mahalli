use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrdersItemsForUpdate {
    pub id: String,
    pub inventory_id: String,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrdersItems {
    pub name: String,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewOrderItem {
    pub price: f64,
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateOrderItem {
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub inventory_id: Option<String>,
    pub price: f64,
    pub quantity: f64,
    pub product_id: String,
}
