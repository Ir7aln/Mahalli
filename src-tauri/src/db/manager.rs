use std::path::Path;

use serde::{Deserialize, Serialize};
use specta::Type;
use system_service::{
    sea_orm::DatabaseConnection as SystemDbConnection, ActivateDatabaseInput, CreateDatabaseInput,
    DatabaseRecord, MutationsService as SystemMutationsService,
    QueriesService as SystemQueriesService,
};
use tenant_service::sea_orm::DatabaseConnection as TenantDbConnection;

use crate::db::{paths::AppPaths, tenant::open_tenant_database};

#[derive(Clone)]
pub struct DatabaseManager {
    paths: AppPaths,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct CreateTenantDatabaseRequest {
    pub name: String,
    pub clone_from_database_id: Option<String>,
    pub make_active: bool,
}

impl DatabaseManager {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn paths(&self) -> &AppPaths {
        &self.paths
    }

    pub async fn open_active_tenant_database(
        &self,
        system_db: &SystemDbConnection,
    ) -> Result<Option<TenantDbConnection>, String> {
        let active = self.get_active_database(system_db).await?;
        match active {
            Some(active) => Ok(Some(open_tenant_database(Path::new(&active.file_path)).await)),
            None => Ok(None),
        }
    }

    pub async fn list_databases(
        &self,
        system_db: &SystemDbConnection,
    ) -> Result<Vec<DatabaseRecord>, String> {
        SystemQueriesService::list_databases(system_db)
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn get_active_database(
        &self,
        system_db: &SystemDbConnection,
    ) -> Result<Option<DatabaseRecord>, String> {
        SystemQueriesService::get_active_database(system_db)
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn create_database(
        &self,
        system_db: &SystemDbConnection,
        input: CreateTenantDatabaseRequest,
    ) -> Result<DatabaseRecord, String> {
        let base_slug = sanitize_slug(&input.name);
        let existing = self.list_databases(system_db).await?;
        let slug = uniquify_slug(&base_slug, &existing);
        let file_name = format!("{slug}.sqlite");
        let file_path = self.paths.tenants_dir.join(&file_name);

        if let Some(source_id) = input.clone_from_database_id.clone() {
            let source = existing
                .iter()
                .find(|database| database.id == source_id)
                .cloned()
                .ok_or_else(|| String::from("source database not found"))?;
            clone_database_file(Path::new(&source.file_path), &file_path)?;
        }

        open_tenant_database(&file_path).await;

        let inserted_id = SystemMutationsService::create_database(
            system_db,
            CreateDatabaseInput {
                name: input.name,
                slug: slug.clone(),
                file_name,
                file_path: file_path.to_string_lossy().to_string(),
                created_from_database_id: input.clone_from_database_id,
                is_active: input.make_active,
            },
        )
        .await
        .map_err(|err| err.to_string())?;

        let created = self
            .list_databases(system_db)
            .await?
            .into_iter()
            .find(|database| database.id == inserted_id)
            .ok_or_else(|| String::from("created database was not found"))?;

        Ok(created)
    }

    pub async fn switch_database(
        &self,
        system_db: &SystemDbConnection,
        database_id: String,
    ) -> Result<(DatabaseRecord, TenantDbConnection), String> {
        SystemMutationsService::activate_database(system_db, ActivateDatabaseInput { id: database_id.clone() })
            .await
            .map_err(|err| err.to_string())?;

        let active = self
            .get_active_database(system_db)
            .await?
            .ok_or_else(|| String::from("active database not found after switch"))?;
        let conn = open_tenant_database(Path::new(&active.file_path)).await;

        Ok((active, conn))
    }
}

fn clone_database_file(source_path: &Path, target_path: &Path) -> Result<(), String> {
    if !source_path.exists() {
        return Err(String::from("source database file does not exist"));
    }
    if target_path.exists() {
        return Err(String::from("target database file already exists"));
    }
    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    std::fs::copy(source_path, target_path).map_err(|err| err.to_string())?;
    Ok(())
}

fn sanitize_slug(input: &str) -> String {
    let slug = input
        .to_lowercase()
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' => ch,
            _ => '-',
        })
        .collect::<String>();

    let slug = slug
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        String::from("database")
    } else {
        slug
    }
}

fn uniquify_slug(base_slug: &str, existing: &[DatabaseRecord]) -> String {
    if existing.iter().all(|database| database.slug != base_slug) {
        return base_slug.to_string();
    }

    let mut index = 2;
    loop {
        let candidate = format!("{base_slug}-{index}");
        if existing.iter().all(|database| database.slug != candidate) {
            return candidate;
        }
        index += 1;
    }
}
