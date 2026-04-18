use tauri::State;

use tenant_service::SeedService;

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn seed_database(state: State<'_, AppState>) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match SeedService::seed_database(&db_conn).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Some(String::from("Database seeded successfully")),
            data: Some(()),
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
