use sea_orm::FromQueryResult;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectProducts {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub selling_price: Option<f64>,
    pub purchase_price: Option<f64>,
    pub inventory: f64,
    pub min_quantity: Option<f64>,
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
    pub price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub selling_price: f64,
    pub purchase_price: f64,
    pub min_quantity: f64,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub selling_price: f64,
    pub purchase_price: f64,
    pub min_quantity: f64,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateProduct {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub selling_price: Option<f64>,
    pub purchase_price: Option<f64>,
    pub min_quantity: Option<f64>,
    pub image: Option<String>,
}
