use crate::{NewQuoteItem, UpdateQuoteItem};
use super::quote_item::{SelectQuotesItems, SelectQuotesItemsForUpdate};
use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::Decimal;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectQuotes {
    pub id: String,
    pub created_at: String,
    pub client_id: String,
    pub full_name: String,
    pub products: i64,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectQuoteDetails {
    pub id: String,
    pub created_at: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewQuote {
    pub client_id: String,
    pub items: Vec<NewQuoteItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateQuote {
    pub id: String,
    pub client_id: String,
    pub items: Vec<UpdateQuoteItem>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct QuotesResponse {
    pub count: u64,
    pub quotes: Vec<SelectQuotes>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct QuoteProductItem {
    pub name: String,
    #[specta(type = f64)]
    pub price: Decimal,
    #[specta(type = f64)]
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct QuoteWithClient {
    pub id: String,
    pub client_id: String,
    pub created_at: String,
    pub identifier: Option<String>,
    pub full_name: String,
    pub items: Vec<SelectQuotesItemsForUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct QuoteDetailsResponse {
    pub id: String,
    pub created_at: String,
    pub identifier: String,
    #[specta(type = f64)]
    pub total: Decimal,
    pub client: QuoteClientInfo,
    pub items: Vec<SelectQuotesItems>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct QuoteClientInfo {
    pub full_name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}
