use serde::{Deserialize, Serialize};
use specta::Type;
use tenant_service::sea_orm::DatabaseConnection as TenantDatabaseConnection;

pub mod clients;
pub mod databases;
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

pub async fn tenant_db_or_fail(
    state: &crate::AppState,
) -> Result<TenantDatabaseConnection, Fail> {
    state.tenant_db().await.map_err(|err| Fail {
        error: Some(err),
        message: Some(String::from("Select a database before using the app.")),
    })
}
