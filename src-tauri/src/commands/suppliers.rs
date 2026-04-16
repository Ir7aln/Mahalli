use tauri::State;

use tenant_service::{
    ListArgs, MutationsService, NewSupplier, QueriesService, Supplier, SupplierSearch,
    SuppliersResponse,
};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_suppliers(
    state: State<'_, AppState>,
    args: ListArgs,
) -> SResult<SuppliersResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::list_suppliers(&db_conn, args).await {
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
pub async fn search_suppliers(
    state: State<'_, AppState>,
    search: String,
) -> SResult<Vec<SupplierSearch>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::search_suppliers(&db_conn, search).await {
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
pub async fn create_supplier(state: State<'_, AppState>, supplier: NewSupplier) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::create_supplier(&db_conn, supplier).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("supplier created successfully")),
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
pub async fn delete_supplier(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::delete_supplier(&db_conn, id).await {
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
pub async fn update_supplier(state: State<'_, AppState>, supplier: Supplier) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::update_supplier(&db_conn, supplier).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("update suppliers success")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
