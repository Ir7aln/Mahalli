use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::Databases, DatabaseRecord};

pub struct QueriesService;

impl QueriesService {
    pub async fn list_databases(
        db: &DatabaseConnection,
    ) -> Result<Vec<DatabaseRecord>, sea_orm::DbErr> {
        let models = Databases::find().all(db).await?;
        Ok(models
            .into_iter()
            .map(|model| DatabaseRecord {
                id: model.id,
                name: model.name,
                slug: model.slug,
                file_name: model.file_name,
                file_path: model.file_path,
                is_active: model.is_active,
                created_from_database_id: model.created_from_database_id,
                created_at: model.created_at.to_string(),
                updated_at: model.updated_at.to_string(),
                last_opened_at: model.last_opened_at.map(|value| value.to_string()),
            })
            .collect())
    }

    pub async fn get_active_database(
        db: &DatabaseConnection,
    ) -> Result<Option<DatabaseRecord>, sea_orm::DbErr> {
        let model = Databases::find()
            .filter(crate::entities::databases::Column::IsActive.eq(true))
            .one(db)
            .await?;

        Ok(model.map(|model| DatabaseRecord {
            id: model.id,
            name: model.name,
            slug: model.slug,
            file_name: model.file_name,
            file_path: model.file_path,
            is_active: model.is_active,
            created_from_database_id: model.created_from_database_id,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            last_opened_at: model.last_opened_at.map(|value| value.to_string()),
        }))
    }
}
