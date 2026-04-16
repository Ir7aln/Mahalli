use tauri::State;

use tenant_service::{
    Client, ClientDetails, ClientSearch, ClientsResponse, ListArgs, MutationsService, NewClient,
    QueriesService,
};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_clients(state: State<'_, AppState>, args: ListArgs) -> SResult<ClientsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::list_clients(&db_conn, args).await {
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
pub async fn search_clients(
    state: State<'_, AppState>,
    search: String,
) -> SResult<Vec<ClientSearch>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::search_clients(&db_conn, search).await {
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
pub async fn get_client(state: State<'_, AppState>, id: String) -> SResult<ClientDetails> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::get_client(&db_conn, id).await {
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
pub async fn create_client(state: State<'_, AppState>, client: NewClient) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::create_client(&db_conn, client).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("client created successfully")),
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
pub async fn delete_client(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::delete_client(&db_conn, id).await {
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
pub async fn update_client(state: State<'_, AppState>, client: Client) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::update_client(&db_conn, client).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("update clients success")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
