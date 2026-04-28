use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreateCreditNote {
    pub invoice_id: String,
    pub reason: Option<String>,
    pub items: Vec<CreditNoteItemInput>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNoteItemInput {
    pub product_id: String,
    pub quantity: f32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNoteResponse {
    pub id: String,
    pub invoice_id: String,
    pub invoice_identifier: Option<String>,
    pub client_id: String,
    pub full_name: String,
    pub identifier: Option<String>,
    pub reason: Option<String>,
    pub created_at: String,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNoteDetailsResponse {
    pub id: String,
    pub invoice_id: String,
    pub invoice_identifier: Option<String>,
    pub client_id: String,
    pub identifier: Option<String>,
    pub reason: Option<String>,
    pub created_at: String,
    pub total: f64,
    pub client: CreditNoteClientInfo,
    pub items: Vec<CreditNoteProductItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNoteClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub ice: Option<String>,
    pub if_number: Option<String>,
    pub rc: Option<String>,
    pub patente: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNoteProductItem {
    pub product_id: String,
    pub name: String,
    #[specta(type = f32)]
    pub quantity: f32,
    #[specta(type = f64)]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListCreditNotesArgs {
    pub limit: i64,
    pub offset: i64,
    pub search: String,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreditNotesListResponse {
    pub count: i64,
    pub notes: Vec<CreditNoteResponse>,
}
