use tauri::State;

use tenant_service::inventory::{
    InventoryResponse, InventoryService, ListInventoryArgs, NewInventory, VoidInventoryArgs,
};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_inventory(
    state: State<'_, AppState>,
    args: ListInventoryArgs,
) -> SResult<InventoryResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InventoryService::list_inventory(&db_conn, args).await {
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
pub async fn create_inventory(
    state: State<'_, AppState>,
    transaction: NewInventory,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InventoryService::create_inventory(&db_conn, transaction).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("inventory created successfully")),
            data: Some(id),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn void_inventory_transaction(
    state: State<'_, AppState>,
    args: VoidInventoryArgs,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InventoryService::void_inventory_transaction(&db_conn, args).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("inventory transaction voided")),
            data: None,
        }),
        Err(err) => Err(err.into()),
    }
}
