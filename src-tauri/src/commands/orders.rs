use tauri::State;

use service::{
    ListArgs, MutationsService, NewOrder, OrderDetailsResponse, OrderProductItem, OrderWithClient,
    OrdersResponse, QueriesService, TransactionService, UpdateOrder, UpdateStatus,
};

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_order_from_quote(state: State<'_, AppState>, id: String) -> SResult<String> {
    let _ = state.db_conn;
    match TransactionService::create_order_from_quote(&state.db_conn, id).await {
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
pub async fn list_orders(state: State<'_, AppState>, args: ListArgs) -> SResult<OrdersResponse> {
    let _ = state.db_conn;
    match QueriesService::list_orders(&state.db_conn, args).await {
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
pub async fn list_order_products(
    state: State<'_, AppState>,
    id: String,
) -> SResult<Vec<OrderProductItem>> {
    let _ = state.db_conn;
    match QueriesService::list_order_products(&state.db_conn, id).await {
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
pub async fn create_order(state: State<'_, AppState>, order: NewOrder) -> SResult<String> {
    let _ = state.db_conn;
    match TransactionService::create_order(&state.db_conn, order).await {
        Ok(id) => Ok(Success {
            error: None,
            message: None,
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
pub async fn update_order(state: State<'_, AppState>, order: UpdateOrder) -> SResult<()> {
    let _ = state.db_conn;
    match TransactionService::update_order(&state.db_conn, order).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update orders success")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_order_status(state: State<'_, AppState>, order: UpdateStatus) -> SResult<()> {
    let _ = state.db_conn;
    match MutationsService::update_order_status(&state.db_conn, order).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update orders success")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_order(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    match MutationsService::delete_order(&state.db_conn, id).await {
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
pub async fn get_order(state: State<'_, AppState>, id: String) -> SResult<OrderWithClient> {
    let _ = state.db_conn;
    match QueriesService::get_order(&state.db_conn, id).await {
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
pub async fn get_order_details(
    state: State<'_, AppState>,
    id: String,
) -> SResult<OrderDetailsResponse> {
    let _ = state.db_conn;
    match QueriesService::get_order_details(&state.db_conn, id).await {
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
