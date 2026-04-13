use crate::{NewOrderItem, UpdateOrderItem};
use super::order_item::{SelectOrdersItems, SelectOrdersItemsForUpdate};
use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectOrders {
    pub id: String,
    pub created_at: String,
    pub client_id: String,
    pub full_name: String,
    pub status: String,
    pub identifier: String,
    pub products: i64,
    pub total: f64,
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
    pub total: f64,
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

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct OrderProductItem {
    pub name: String,
    pub price: f64,
    pub quantity: f64,
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
    pub total: f64,
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
