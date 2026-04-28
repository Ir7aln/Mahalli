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
    pub client_id: String,
    pub identifier: Option<String>,
    pub reason: Option<String>,
    pub created_at: String,
    pub total: f64,
}
