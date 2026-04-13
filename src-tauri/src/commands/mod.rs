use serde::{Deserialize, Serialize};
use specta::Type;

pub mod clients;
pub mod dashboard;
pub mod inventory;
pub mod invoice_items;
pub mod invoices;
pub mod order_items;
pub mod orders;
pub mod products;
pub mod quote_items;
pub mod quotes;
pub mod suppliers;
pub mod templates;

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Success<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Fail {
    pub error: Option<String>,
    pub message: Option<String>,
}

pub type SResult<T> = Result<Success<T>, Fail>;
