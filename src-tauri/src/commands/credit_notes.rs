use tauri::State;

use tenant_service::credit_notes::{CreateCreditNote, CreditNoteResponse, CreditNotesService};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_credit_note(
    state: State<'_, AppState>,
    credit_note: CreateCreditNote,
) -> SResult<CreditNoteResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match CreditNotesService::create_credit_note(&db_conn, credit_note).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}
