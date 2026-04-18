use sea_orm::entity::prelude::Decimal;
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
    #[specta(type = f64)]
    pub selling_price: Option<Decimal>,
    #[specta(type = f64)]
    pub purchase_price: Option<Decimal>,
    #[specta(type = f64)]
    pub inventory: Decimal,
    #[specta(type = f64)]
    pub min_quantity: Option<Decimal>,
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
    #[specta(type = f64)]
    pub price: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    #[specta(type = f64)]
    pub selling_price: Decimal,
    #[specta(type = f64)]
    pub purchase_price: Decimal,
    #[specta(type = f64)]
    pub min_quantity: Decimal,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[specta(type = f64)]
    pub selling_price: Decimal,
    #[specta(type = f64)]
    pub purchase_price: Decimal,
    #[specta(type = f64)]
    pub min_quantity: Decimal,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateProduct {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    #[specta(type = f64)]
    pub selling_price: Option<Decimal>,
    #[specta(type = f64)]
    pub purchase_price: Option<Decimal>,
    #[specta(type = f64)]
    pub min_quantity: Option<Decimal>,
    pub image: Option<String>,
}
