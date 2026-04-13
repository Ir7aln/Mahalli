use crate::{NewInvoiceItem, UpdateInvoiceItem};
use super::invoice_item::{SelectInvoicesItems, SelectInvoicesItemsForUpdate};
use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::Decimal;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoices {
    pub id: String,
    pub created_at: String,
    #[specta(type = f64)]
    pub paid_amount: Decimal,
    pub client_id: String,
    pub full_name: String,
    pub status: String,
    pub identifier: String,
    pub products: i64,
    #[specta(type = f64)]
    pub total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoiceDetails {
    pub id: String,
    pub order_id: String,
    pub created_at: String,
    #[specta(type = f64)]
    pub paid_amount: Decimal,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInvoice {
    pub client_id: String,
    pub order_id: Option<String>,
    pub status: String,
    #[specta(type = f64)]
    pub paid_amount: Decimal,
    pub items: Vec<NewInvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateInvoice {
    pub id: String,
    pub client_id: String,
    pub status: String,
    #[specta(type = f64)]
    pub paid_amount: Decimal,
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
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceWithClient {
    pub id: String,
    pub client_id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: Option<String>,
    #[specta(type = f64)]
    pub paid_amount: Decimal,
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
    #[specta(type = f64)]
    pub paid_amount: Decimal,
    #[specta(type = f64)]
    pub total: Decimal,
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
