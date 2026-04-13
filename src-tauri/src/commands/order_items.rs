use service::MutationsService;
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn delete_order_item(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let res = MutationsService::delete_order_item(&state.db_conn, id).await;
    match res {
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

