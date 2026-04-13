use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectClients {
    pub id: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
    pub credit: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewClient {
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Client {
    pub id: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UpdateClient {
    pub id: String,
    pub full_name: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ClientsResponse {
    pub count: u64,
    pub clients: Vec<SelectClients>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Type)]
pub struct ClientSearch {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ClientDetails {
    pub id: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
}
