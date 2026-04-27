use tauri::State;
use tenant_service::orders::OrdersService;

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn delete_order_item(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    let res = OrdersService::delete_order_item(&db_conn, id).await;
    match res {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}
