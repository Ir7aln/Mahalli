use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListInvoicesArgs {
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
pub struct SelectInvoices {
    pub id: String,
    pub created_at: String,
    #[specta(type = f64)]
    pub paid_amount: f64,
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
    pub products: i64,
    #[specta(type = f64)]
    pub total: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoiceDetails {
    pub id: String,
    pub order_id: String,
    pub created_at: String,
    #[specta(type = f64)]
    pub paid_amount: f64,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInvoiceItem {
    #[specta(type = f64)]
    pub price: f64,
    #[specta(type = f64)]
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateInvoiceItem {
    pub id: Option<String>,
    pub invoice_id: Option<String>,
    pub inventory_id: Option<String>,
    #[specta(type = f64)]
    pub price: f64,
    #[specta(type = f64)]
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInvoice {
    pub client_id: String,
    pub order_id: Option<String>,
    pub status: String,
    pub items: Vec<NewInvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateInvoice {
    pub id: String,
    pub client_id: String,
    pub status: String,
    pub items: Vec<UpdateInvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoicesResponse {
    pub count: u64,
    pub invoices: Vec<SelectInvoices>,
}

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct UpdateInvoiceStatus {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct InvoiceProductItem {
    pub name: String,
    #[specta(type = f64)]
    pub price: f64,
    #[specta(type = f64)]
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceWithClient {
    pub id: String,
    pub client_id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: Option<String>,
    #[specta(type = f64)]
    pub paid_amount: f64,
    #[specta(type = f64)]
    pub total: f64,
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub items: Vec<SelectInvoicesItemsForUpdate>,
    pub payments: Vec<SelectInvoicePayment>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceDetailsResponse {
    pub id: String,
    pub created_at: String,
    pub status: String,
    pub identifier: String,
    #[specta(type = f64)]
    pub paid_amount: f64,
    #[specta(type = f64)]
    pub total: f64,
    pub client: InvoiceClientInfo,
    pub items: Vec<SelectInvoicesItems>,
    pub payments: Vec<SelectInvoicePayment>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InvoiceClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoicesItemsForUpdate {
    pub id: String,
    pub inventory_id: Option<String>,
    pub name: String,
    #[specta(type = f64)]
    pub price: f64,
    #[specta(type = f64)]
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoicesItems {
    pub name: String,
    #[specta(type = f64)]
    pub price: f64,
    #[specta(type = f64)]
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoicePayment {
    pub id: String,
    pub payment_date: String,
    pub description: Option<String>,
    #[specta(type = f64)]
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AddInvoicePayment {
    pub invoice_id: String,
    pub payment_date: String,
    pub description: Option<String>,
    #[specta(type = f64)]
    pub amount: f64,
}
