use tauri::State;

use tenant_service::services::templates::{service::MutationsService, types::NewTemplate};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_template(state: State<'_, AppState>, template: NewTemplate) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::create_template(&db_conn, template).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("template created successfully")),
            data: Some(id),
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
