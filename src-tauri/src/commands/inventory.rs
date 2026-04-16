use tauri::State;

use tenant_service::{InventoryResponse, ListArgs, MutationsService, NewInventory, QueriesService};

use crate::{commands::Fail, AppState};

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_inventory(
    state: State<'_, AppState>,
    args: ListArgs,
) -> SResult<InventoryResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::list_inventory(&db_conn, args).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn create_inventory(
    state: State<'_, AppState>,
    transaction: NewInventory,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::create_inventory(&db_conn, transaction).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("inventory created successfully")),
            data: Some(id),
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_inventory(state: State<'_, AppState>, id: String) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::delete_inventory(&db_conn, id).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("inventory deleted successfully")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
