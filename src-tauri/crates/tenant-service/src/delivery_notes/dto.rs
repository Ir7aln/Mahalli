use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListDeliveryNotesArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub status: Option<String>,
    pub created_from: Option<String>,
    pub created_to: Option<String>,
    pub total_min: Option<f64>,
    pub total_max: Option<f64>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectDeliveryNotes {
    pub id: String,
    pub created_at: String,
    pub client_id: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub ice: Option<String>,
    pub if_number: Option<String>,
    pub rc: Option<String>,
    pub patente: Option<String>,
    pub status: String,
    pub identifier: String,
    pub order_id: String,
    pub order_identifier: String,
    pub products: i64,
    #[specta(type = f32)]
    pub total: f32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeliveryNotesResponse {
    pub count: u64,
    pub delivery_notes: Vec<SelectDeliveryNotes>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeliveryNoteDetailsResponse {
    pub id: String,
    pub created_at: String,
    pub client_id: String,
    pub identifier: Option<String>,
    pub order_id: String,
    pub order_identifier: Option<String>,
    #[specta(type = f32)]
    pub total: f32,
    pub client: DeliveryNoteClientInfo,
    pub items: Vec<DeliveryNoteProductDetailItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeliveryNoteClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub ice: Option<String>,
    pub if_number: Option<String>,
    pub rc: Option<String>,
    pub patente: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct DeliveryNoteProductItem {
    pub name: String,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeliveryNoteProductDetailItem {
    pub product_id: String,
    pub name: String,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
}
