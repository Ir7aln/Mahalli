use tauri::State;

use tenant_service::dashboard::{DashboardService, FinancialMetricsResponse};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

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
        Err(err) => Err(err.into()),
    }
}
