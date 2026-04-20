use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ListSuppliersArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectSuppliers {
    pub id: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewSupplier {
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Supplier {
    pub id: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SuppliersResponse {
    pub count: u64,
    pub suppliers: Vec<SelectSuppliers>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct SupplierSearch {
    pub label: String,
    pub value: String,
}
