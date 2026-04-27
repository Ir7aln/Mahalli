use tauri::State;

use tenant_service::column_preferences::{ColumnPreference, ColumnPreferencesService, SaveColumnPreferenceArgs};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn get_column_preferences(
    state: State<'_, AppState>,
    page: String,
) -> SResult<Option<ColumnPreference>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ColumnPreferencesService::get_preferences(&db_conn, &page).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn save_column_preferences(
    state: State<'_, AppState>,
    args: SaveColumnPreferenceArgs,
) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ColumnPreferencesService::save_preferences(&db_conn, args).await {
        Ok(_) => Ok(Success {
            error: None,
            message: None,
            data: Some(()),
        }),
        Err(err) => Err(err.into()),
    }
}
