use sea_orm::entity::prelude::Decimal;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListOrdersArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrders {
    pub id: String,
    pub created_at: String,
    pub client_id: String,
    pub full_name: String,
    pub status: String,
    pub identifier: String,
    pub products: i64,
    #[specta(type = f64)]
    pub total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrderDetails {
    pub id: String,
    pub created_at: String,
    pub full_name: String,
    pub identifier: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    #[specta(type = f64)]
    pub total: Decimal,
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

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewOrder {
    pub client_id: String,
    pub status: String,
    pub items: Vec<NewOrderItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateOrder {
    pub id: String,
    pub client_id: String,
    pub status: String,
    pub items: Vec<UpdateOrderItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct OrdersResponse {
    pub count: u64,
    pub orders: Vec<SelectOrders>,
}

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct UpdateOrderStatus {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct OrderProductItem {
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct OrderWithClient {
    pub id: String,
    pub client_id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: Option<String>,
    pub full_name: String,
    pub items: Vec<SelectOrdersItemsForUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct OrderDetailsResponse {
    pub id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: Decimal,
    pub client: OrderClientInfo,
    pub items: Vec<SelectOrdersItems>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct OrderClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

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
