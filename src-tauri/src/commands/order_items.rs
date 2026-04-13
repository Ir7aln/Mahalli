use service::{MutationsService, NewOrderItem, OrderItem};
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_order_item(state: State<'_, AppState>, item: NewOrderItem) -> SResult<String> {
    let _ = state.db_conn;
    let res = MutationsService::create_order_item(&state.db_conn, item).await;
    match res {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_order_item(state: State<'_, AppState>, item: OrderItem) -> SResult<()> {
    let _ = state.db_conn;
    let res = MutationsService::update_order_item(&state.db_conn, item).await;
    match res {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_order_item(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    let res = MutationsService::delete_order_item(&state.db_conn, id).await;
    match res {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}
