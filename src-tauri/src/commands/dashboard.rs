use tauri::State;

use service::{QueriesService, SelectTransaction, SelectTops, SelectTopProducts, StatusCountResponse, FinancialMetricsResponse};

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_inventory_stats(state: State<'_, AppState>) -> SResult<Vec<SelectTransaction>> {
    let _ = state.db_conn;
    match QueriesService::list_inventory_stats(&state.db_conn).await {
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
pub async fn list_top_clients(state: State<'_, AppState>) -> SResult<Vec<SelectTops>> {
    let _ = state.db_conn;
    match QueriesService::list_top_clients(&state.db_conn).await {
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
pub async fn list_top_suppliers(state: State<'_, AppState>) -> SResult<Vec<SelectTops>> {
    let _ = state.db_conn;
    match QueriesService::list_top_suppliers(&state.db_conn).await {
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
pub async fn list_top_products(state: State<'_, AppState>) -> SResult<Vec<SelectTopProducts>> {
    let _ = state.db_conn;
    match QueriesService::list_top_products(&state.db_conn).await {
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
pub async fn list_status_count(state: State<'_, AppState>) -> SResult<StatusCountResponse> {
    let _ = state.db_conn;
    match QueriesService::list_status_count(&state.db_conn).await {
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
pub async fn list_financial_metrics(state: State<'_, AppState>) -> SResult<FinancialMetricsResponse> {
    let _ = state.db_conn;
    match QueriesService::list_financial_metrics(&state.db_conn).await {
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
