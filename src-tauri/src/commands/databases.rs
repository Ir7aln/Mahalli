use tauri::State;

use serde::{Deserialize, Serialize};
use specta::Type;
use system_service::types::DatabaseRecord;

use crate::{db::manager::CreateTenantDatabaseRequest, AppState};

use super::{Fail, SResult, Success};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DatabaseBootstrapStatus {
    pub databases: Vec<DatabaseRecord>,
    pub active_database: Option<DatabaseRecord>,
    pub has_any_database: bool,
    pub has_active_database: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn list_databases(state: State<'_, AppState>) -> SResult<Vec<DatabaseRecord>> {
    match state.db_manager().list_databases(state.system_db()).await {
        Ok(databases) => Ok(Success {
            error: None,
            message: None,
            data: Some(databases),
        }),
        Err(err) => Err(Fail {
            error: Some(err),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_database_bootstrap_status(
    state: State<'_, AppState>,
) -> SResult<DatabaseBootstrapStatus> {
    let databases = state.db_manager().list_databases(state.system_db()).await;
    let active_database = state
        .db_manager()
        .get_active_database(state.system_db())
        .await;

    match (databases, active_database) {
        (Ok(databases), Ok(active_database)) => Ok(Success {
            error: None,
            message: None,
            data: Some(DatabaseBootstrapStatus {
                has_any_database: !databases.is_empty(),
                has_active_database: active_database.is_some(),
                databases,
                active_database,
            }),
        }),
        (Err(err), _) | (_, Err(err)) => Err(Fail {
            error: Some(err),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_active_database(state: State<'_, AppState>) -> SResult<DatabaseRecord> {
    match state
        .db_manager()
        .get_active_database(state.system_db())
        .await
    {
        Ok(Some(database)) => Ok(Success {
            error: None,
            message: None,
            data: Some(database),
        }),
        Ok(None) => Err(Fail {
            error: Some(String::from("No active database found")),
            message: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn create_database(
    state: State<'_, AppState>,
    input: CreateTenantDatabaseRequest,
) -> SResult<DatabaseRecord> {
    match state
        .db_manager()
        .create_database(state.system_db(), input)
        .await
    {
        Ok(database) => {
            if database.is_active {
                let tenant_db = state
                    .db_manager()
                    .open_active_tenant_database(state.system_db())
                    .await
                    .map_err(|err| Fail {
                        error: Some(err),
                        message: None,
                    })?;
                state.set_tenant_db(tenant_db).await;
            }

            Ok(Success {
                error: None,
                message: Some(String::from("database created successfully")),
                data: Some(database),
            })
        }
        Err(err) => Err(Fail {
            error: Some(err),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn switch_database(state: State<'_, AppState>, id: String) -> SResult<DatabaseRecord> {
    match state
        .db_manager()
        .switch_database(state.system_db(), id)
        .await
    {
        Ok((database, conn)) => {
            state.set_tenant_db(Some(conn)).await;
            Ok(Success {
                error: None,
                message: Some(String::from("database switched successfully")),
                data: Some(database),
            })
        }
        Err(err) => Err(Fail {
            error: Some(err),
            message: None,
        }),
    }
}
