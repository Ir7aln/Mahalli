use tauri::State;

use tenant_service::dashboard::{
    DashboardService, FinancialMetricsResponse, SelectTopProducts, SelectTops, SelectTransaction,
    StatusCountResponse,
};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_inventory_stats(state: State<'_, AppState>) -> SResult<Vec<SelectTransaction>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_inventory_stats(&db_conn).await {
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
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_top_clients(&db_conn).await {
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
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_top_suppliers(&db_conn).await {
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
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_top_products(&db_conn).await {
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
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_status_count(&db_conn).await {
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
pub async fn list_financial_metrics(
    state: State<'_, AppState>,
) -> SResult<FinancialMetricsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match DashboardService::list_financial_metrics(&db_conn).await {
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
