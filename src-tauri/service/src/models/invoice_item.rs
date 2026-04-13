use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::Decimal;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoicesItemsForUpdate {
    pub id: String,
    pub inventory_id: Option<String>,
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectInvoicesItems {
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewInvoiceItem {
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateInvoiceItem {
    pub id: Option<String>,
    pub invoice_id: Option<String>,
    pub inventory_id: Option<String>,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
    pub product_id: String,
}
