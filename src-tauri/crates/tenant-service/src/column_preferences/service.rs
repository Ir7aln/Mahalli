use sea_orm::{
    entity::prelude::*, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tenant_entity::user_column_preferences::{self, Entity as UserColumnPreferences};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ColumnPreference {
    pub page: String,
    pub visible_columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SaveColumnPreferenceArgs {
    pub page: String,
    pub visible_columns: Vec<String>,
}

pub struct ColumnPreferencesService;

impl ColumnPreferencesService {
    pub async fn get_preferences(
        db: &DbConn,
        page: &str,
    ) -> Result<Option<ColumnPreference>, DbErr> {
        let preference = UserColumnPreferences::find()
            .filter(user_column_preferences::Column::Page.eq(page))
            .one(db)
            .await?;

        Ok(preference.map(|p| ColumnPreference {
            page: p.page,
            visible_columns: serde_json::from_str(&p.visible_columns)
                .unwrap_or_default(),
        }))
    }

    pub async fn save_preferences(
        db: &DbConn,
        args: SaveColumnPreferenceArgs,
    ) -> Result<(), DbErr> {
        let visible_columns_json = serde_json::to_string(&args.visible_columns)
            .unwrap_or_else(|_| "[]".to_string());

        // Check if preference already exists
        let existing = UserColumnPreferences::find()
            .filter(user_column_preferences::Column::Page.eq(&args.page))
            .one(db)
            .await?;

        if let Some(existing) = existing {
            // Update existing
            let mut active_model: user_column_preferences::ActiveModel = existing.into();
            active_model.visible_columns = Set(visible_columns_json);
            active_model.updated_at = Set(chrono::Utc::now().naive_utc());
            active_model.update(db).await?;
        } else {
            // Insert new
            let new_preference = user_column_preferences::ActiveModel {
                page: Set(args.page),
                visible_columns: Set(visible_columns_json),
                ..Default::default()
            };
            new_preference.insert(db).await?;
        }

        Ok(())
    }
}
