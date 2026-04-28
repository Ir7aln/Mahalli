use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SellerProfileDTO {
    pub id: String,
    pub legal_name: String,
    pub commercial_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub ice: Option<String>,
    pub if_number: Option<String>,
    pub rc: Option<String>,
    pub patente: Option<String>,
    pub logo: Option<String>,
    pub default_currency: String,
    pub default_payment_terms_days: i32,
    pub invoice_footer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UpdateSellerProfileDTO {
    pub legal_name: Option<String>,
    pub commercial_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub ice: Option<String>,
    pub if_number: Option<String>,
    pub rc: Option<String>,
    pub patente: Option<String>,
    pub logo: Option<String>,
    pub default_currency: Option<String>,
    pub default_payment_terms_days: Option<i32>,
    pub invoice_footer: Option<String>,
}

impl From<system_entity::seller_profile::Model> for SellerProfileDTO {
    fn from(model: system_entity::seller_profile::Model) -> Self {
        SellerProfileDTO {
            id: model.id,
            legal_name: model.legal_name,
            commercial_name: model.commercial_name,
            address: model.address,
            city: model.city,
            phone_number: model.phone_number,
            email: model.email,
            ice: model.ice,
            if_number: model.if_number,
            rc: model.rc,
            patente: model.patente,
            logo: model.logo,
            default_currency: model.default_currency,
            default_payment_terms_days: model.default_payment_terms_days,
            invoice_footer: model.invoice_footer,
        }
    }
}
