use serde_json::Value;
use tauri::State;

use service::{Client, ListArgs, MutationsService, NewClient, QueriesService};

use crate::jobs::{EntityEnum, ImageProcessorJob};
use crate::AppState;

use super::{Fail, SResult, Seccess};

#[tauri::command]
pub async fn list_clients(state: State<'_, AppState>, args: ListArgs) -> SResult<Value> {
    let _ = state.db_conn;
    match QueriesService::list_clients(&state.db_conn, args).await {
        Ok(res) => Ok(Seccess {
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
pub async fn search_clients(state: State<'_, AppState>, search: String) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    match QueriesService::search_clients(&state.db_conn, search).await {
        Ok(res) => Ok(Seccess {
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
pub async fn get_client(state: State<'_, AppState>, id: String) -> SResult<Value> {
    let _ = state.db_conn;
    match QueriesService::get_client(&state.db_conn, id).await {
        Ok(res) => Ok(Seccess {
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
pub async fn create_client(state: State<'_, AppState>, client: NewClient) -> SResult<String> {
    let _ = state.db_conn;
    let image = client.image.clone();
    match MutationsService::create_client(&state.db_conn, client).await {
        Ok(id) => {
            match image {
                Some(data) => {
                    let job = ImageProcessorJob {
                        id: id.clone(),
                        entity: EntityEnum::CLIENT,
                        data,
                    };
                    state
                        .job_storage
                        .push_job(job)
                        .await
                        .expect("error pushing the job");
                }
                None => {}
            }
            Ok(Seccess::<String> {
                error: None,
                message: Option::Some(String::from("client created successfully")),
                data: Some(id),
            })
        }
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}

#[tauri::command]
pub async fn delete_client(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    match MutationsService::delete_client(&state.db_conn, id).await {
        Ok(res) => Ok(Seccess {
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
pub async fn update_client(state: State<'_, AppState>, client: Client) -> SResult<String> {
    let _ = state.db_conn;
    match MutationsService::update_client(&state.db_conn, client).await {
        Ok(_) => Ok(Seccess::<String> {
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
