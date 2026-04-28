use chrono::Utc;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, Set};
use system_entity::prelude::*;

use super::dto::{SellerProfileDTO, UpdateSellerProfileDTO};

pub struct SellerProfileService;

impl SellerProfileService {
    pub async fn get_or_create_profile(db: &sea_orm::DbConn) -> Result<SellerProfileDTO, DbErr> {
        let existing = SellerProfile::find().one(db).await?;

        if let Some(profile) = existing {
            return Ok(profile.into());
        }

        let new_profile = system_entity::seller_profile::ActiveModel {
            id: Set(ulid::Ulid::new().to_string()),
            legal_name: Set("Your Company Name".to_string()),
            default_currency: Set("MAD".to_string()),
            default_payment_terms_days: Set(30),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let inserted: system_entity::seller_profile::Model = new_profile.insert(db).await?;
        Ok(inserted.into())
    }

    pub async fn get_profile(db: &sea_orm::DbConn) -> Result<Option<SellerProfileDTO>, DbErr> {
        let profile = SellerProfile::find().one(db).await?;
        Ok(profile.map(|p| p.into()))
    }

    pub async fn update_profile(
        db: &sea_orm::DbConn,
        update: UpdateSellerProfileDTO,
    ) -> Result<SellerProfileDTO, DbErr> {
        let existing = SellerProfile::find()
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Seller profile not found".to_string()))?;

        let mut active_model = existing.into_active_model();

        if let Some(legal_name) = update.legal_name {
            active_model.legal_name = Set(legal_name);
        }
        if let Some(commercial_name) = update.commercial_name {
            active_model.commercial_name = Set(Some(commercial_name));
        }
        if let Some(address) = update.address {
            active_model.address = Set(Some(address));
        }
        if let Some(city) = update.city {
            active_model.city = Set(Some(city));
        }
        if let Some(phone_number) = update.phone_number {
            active_model.phone_number = Set(Some(phone_number));
        }
        if let Some(email) = update.email {
            active_model.email = Set(Some(email));
        }
        if let Some(ice) = update.ice {
            active_model.ice = Set(Some(ice));
        }
        if let Some(if_number) = update.if_number {
            active_model.if_number = Set(Some(if_number));
        }
        if let Some(rc) = update.rc {
            active_model.rc = Set(Some(rc));
        }
        if let Some(patente) = update.patente {
            active_model.patente = Set(Some(patente));
        }
        if let Some(logo) = update.logo {
            active_model.logo = Set(Some(logo));
        }
        if let Some(default_currency) = update.default_currency {
            active_model.default_currency = Set(default_currency);
        }
        if let Some(default_payment_terms_days) = update.default_payment_terms_days {
            active_model.default_payment_terms_days = Set(default_payment_terms_days);
        }
        if let Some(invoice_footer) = update.invoice_footer {
            active_model.invoice_footer = Set(Some(invoice_footer));
        }

        active_model.updated_at = Set(Utc::now().naive_utc());

        let updated: system_entity::seller_profile::Model = active_model.update(db).await?;
        Ok(updated.into())
    }
}
