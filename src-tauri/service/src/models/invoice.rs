use crate::{NewInvoiceItem, UpdateInvoiceItem};
use super::invoice_item::{SelectInvoicesItems, SelectInvoicesItemsForUpdate};
use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoices {
    pub id: String,
    pub created_at: String,
    pub paid_amount: f64,
    pub client_id: String,
    pub full_name: String,
    pub status: String,
    pub identifier: String,
    pub products: i64,
    pub total: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoiceDetails {
    pub id: String,
    pub order_id: String,
    pub created_at: String,
    pub paid_amount: f64,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub identifier: String,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInvoice {
    pub client_id: String,
    pub order_id: Option<String>,
    pub status: String,
    pub paid_amount: f64,
    pub items: Vec<NewInvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateInvoice {
    pub id: String,
    pub client_id: String,
    pub status: String,
    pub paid_amount: f64,
    pub items: Vec<UpdateInvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoicesResponse {
    pub count: u64,
    pub invoices: Vec<SelectInvoices>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct InvoiceProductItem {
    pub name: String,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceWithClient {
    pub id: String,
    pub client_id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: Option<String>,
    pub paid_amount: f64,
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub items: Vec<SelectInvoicesItemsForUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceDetailsResponse {
    pub id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: String,
    pub paid_amount: f64,
    pub total: f64,
    pub client: InvoiceClientInfo,
    pub items: Vec<SelectInvoicesItems>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}
