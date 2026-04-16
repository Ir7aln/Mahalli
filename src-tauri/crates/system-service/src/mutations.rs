use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::{databases, DatabaseActiveModel, Databases}, ActivateDatabaseInput, CreateDatabaseInput};

pub struct MutationsService;

impl MutationsService {
    pub async fn create_database(
        db: &DatabaseConnection,
        input: CreateDatabaseInput,
    ) -> Result<String, sea_orm::DbErr> {
        let now = chrono::Utc::now().naive_utc().to_string();

        if input.is_active {
            Self::clear_active_database(db).await?;
        }

        let model = DatabaseActiveModel {
            id: ActiveValue::Set(ulid::Ulid::new().to_string()),
            name: ActiveValue::Set(input.name),
            slug: ActiveValue::Set(input.slug),
            file_name: ActiveValue::Set(input.file_name),
            file_path: ActiveValue::Set(input.file_path),
            is_active: ActiveValue::Set(input.is_active),
            created_from_database_id: ActiveValue::Set(input.created_from_database_id),
            created_at: ActiveValue::Set(now.clone()),
            updated_at: ActiveValue::Set(now),
            last_opened_at: ActiveValue::Set(None),
        };

        Ok(model.insert(db).await?.id)
    }

    pub async fn activate_database(
        db: &DatabaseConnection,
        input: ActivateDatabaseInput,
    ) -> Result<(), sea_orm::DbErr> {
        let now = chrono::Utc::now().naive_utc().to_string();
        Self::clear_active_database(db).await?;

        if let Some(model) = Databases::find_by_id(input.id).one(db).await? {
            let mut active_model: DatabaseActiveModel = model.into();
            active_model.is_active = ActiveValue::Set(true);
            active_model.updated_at = ActiveValue::Set(now.clone());
            active_model.last_opened_at = ActiveValue::Set(Some(now));
            active_model.update(db).await?;
        }

        Ok(())
    }

    async fn clear_active_database(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        let active_models = Databases::find()
            .filter(databases::Column::IsActive.eq(true))
            .all(db)
            .await?;

        for model in active_models {
            let mut active_model: DatabaseActiveModel = model.into();
            active_model.is_active = ActiveValue::Set(false);
            active_model.update(db).await?;
        }

        Ok(())
    }
}
