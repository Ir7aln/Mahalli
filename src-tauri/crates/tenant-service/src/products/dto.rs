use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListProductsArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectProducts {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub description: Option<String>,
    pub image: Option<String>,
    #[specta(type = f32)]
    pub selling_price: Option<f32>,
    #[specta(type = f32)]
    pub purchase_price: Option<f32>,
    #[specta(type = f32)]
    pub inventory: f32,
    #[specta(type = f32)]
    pub min_quantity: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct ProductsResponse {
    pub count: u64,
    pub products: Vec<SelectProducts>,
}

#[derive(Deserialize, Serialize, Debug, FromQueryResult, Type)]
pub struct ProductSearch {
    pub label: String,
    pub value: String,
    #[specta(type = f32)]
    pub price: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    #[specta(type = f32)]
    pub selling_price: f32,
    #[specta(type = f32)]
    pub purchase_price: f32,
    #[specta(type = f32)]
    pub min_quantity: f32,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[specta(type = f32)]
    pub selling_price: f32,
    #[specta(type = f32)]
    pub purchase_price: f32,
    #[specta(type = f32)]
    pub min_quantity: f32,
    pub image: Option<String>,
}
