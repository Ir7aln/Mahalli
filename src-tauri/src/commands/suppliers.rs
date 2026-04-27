use tauri::State;

use tenant_service::suppliers::{
    ListSuppliersArgs, NewSupplier, Supplier, SupplierSearch, SuppliersResponse, SuppliersService,
};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_suppliers(
    state: State<'_, AppState>,
    args: ListSuppliersArgs,
) -> SResult<SuppliersResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SuppliersService::list_suppliers(&db_conn, args).await {
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
pub async fn search_suppliers(
    state: State<'_, AppState>,
    search: String,
) -> SResult<Vec<SupplierSearch>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SuppliersService::search_suppliers(&db_conn, search).await {
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
pub async fn create_supplier(state: State<'_, AppState>, supplier: NewSupplier) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SuppliersService::create_supplier(&db_conn, supplier).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("supplier created successfully")),
            data: Some(id),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_supplier(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SuppliersService::delete_supplier(&db_conn, id).await {
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
pub async fn update_supplier(state: State<'_, AppState>, supplier: Supplier) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SuppliersService::update_supplier(&db_conn, supplier).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("update suppliers success")),
            data: None,
        }),
        Err(err) => Err(err.into()),
    }
}
