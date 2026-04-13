use tauri::State;

use service::{
    ListArgs, MutationsService, NewSupplier, QueriesService, Supplier, SupplierSearch,
    SuppliersResponse,
};

use crate::jobs::{EntityEnum, ImageProcessorJob};

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_suppliers(
    state: State<'_, AppState>,
    args: ListArgs,
) -> SResult<SuppliersResponse> {
    let _ = state.db_conn;
    match QueriesService::list_suppliers(&state.db_conn, args).await {
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
    let _ = state.db_conn;
    match QueriesService::search_suppliers(&state.db_conn, search).await {
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
    let _ = state.db_conn;
    let image = supplier.image.clone();
    match MutationsService::create_supplier(&state.db_conn, supplier).await {
        Ok(id) => {
            match image {
                Some(data) => {
                    let job = ImageProcessorJob {
                        id: id.clone(),
                        entity: EntityEnum::SUPPLIER,
                        data,
                    };
                    state
                        .job_storage
                        .push_job(job)
                        .await
                        .expect("error pushing the job");
                }
                None => {}
            }
            Ok(Success::<String> {
                error: None,
                message: Option::Some(String::from("supplier created successfully")),
                data: Some(id),
            })
        }
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_supplier(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    match MutationsService::delete_supplier(&state.db_conn, id).await {
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
    let _ = state.db_conn;
    match MutationsService::update_supplier(&state.db_conn, supplier).await {
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
