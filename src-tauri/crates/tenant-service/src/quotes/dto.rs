use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListQuotesArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub created_from: Option<String>,
    pub created_to: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectQuotes {
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
    pub products: i64,
    pub identifier: String,
    #[specta(type = f32)]
    pub total: f32,
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
    #[specta(type = f32)]
    pub total: f32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewQuoteItem {
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateQuoteItem {
    pub id: Option<String>,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
    pub product_id: String,
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
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
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
    #[specta(type = f32)]
    pub total: f32,
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

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectQuotesItemsForUpdate {
    pub id: String,
    pub name: String,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectQuotesItems {
    pub name: String,
    #[specta(type = f32)]
    pub price: f32,
    #[specta(type = f32)]
    pub quantity: f32,
}
