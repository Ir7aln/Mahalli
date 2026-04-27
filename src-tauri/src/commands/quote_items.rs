use tauri::State;

use tenant_service::quotes::QuotesService;

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn delete_quote_item(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::delete_quote_item(&db_conn, id).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}
