use sea_orm::entity::prelude::Decimal;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrdersItemsForUpdate {
    pub id: String,
    pub inventory_id: String,
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrdersItems {
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewOrderItem {
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateOrderItem {
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub inventory_id: Option<String>,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}
