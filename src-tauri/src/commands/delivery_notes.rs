use tauri::State;

use tenant_service::delivery_notes::{
    DeliveryNoteProductItem, DeliveryNotesResponse, DeliveryNotesService, ListDeliveryNotesArgs,
};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_delivery_notes(
    state: State<'_, AppState>,
    args: ListDeliveryNotesArgs,
) -> SResult<DeliveryNotesResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match DeliveryNotesService::list_delivery_notes(&db_conn, args).await {
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
pub async fn list_delivery_note_products(
    state: State<'_, AppState>,
    id: String,
) -> SResult<Vec<DeliveryNoteProductItem>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match DeliveryNotesService::list_delivery_note_products(&db_conn, id).await {
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
pub async fn create_delivery_note_from_order(
    state: State<'_, AppState>,
    id: String,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match DeliveryNotesService::create_delivery_note_from_order(&db_conn, id).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}
