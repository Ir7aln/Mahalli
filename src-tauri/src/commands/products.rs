use tauri::State;

use service::{ListArgs, MutationsService, NewProduct, Product, ProductSearch, ProductsResponse, QueriesService};

use crate::jobs::{EntityEnum, ImageProcessorJob};

use crate::AppState;

use super::{Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_products(state: State<'_, AppState>, args: ListArgs) -> SResult<ProductsResponse> {
    let _ = state.db_conn;
    match QueriesService::list_products(&state.db_conn, args).await {
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
pub async fn search_products(state: State<'_, AppState>, search: String) -> SResult<Vec<ProductSearch>> {
    let _ = state.db_conn;
    match QueriesService::search_products(&state.db_conn, search).await {
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
pub async fn create_product(state: State<'_, AppState>, product: NewProduct) -> SResult<String> {
    let _ = state.db_conn;
    let image = product.image.clone();
    match MutationsService::create_product(&state.db_conn, product).await {
        Ok(id) => {
            match image {
                Some(data) => {
                    let job = ImageProcessorJob {
                        id: id.clone(),
                        entity: EntityEnum::PRODUCT,
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
            Ok(Success::<String> {
                error: None,
                message: Option::Some(String::from("product created successfully")),
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
#[specta::specta]
pub async fn delete_product(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    match MutationsService::delete_product(&state.db_conn, id).await {
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
pub async fn update_product(state: State<'_, AppState>, product: Product) -> SResult<String> {
    let _ = state.db_conn;
    match MutationsService::update_product(&state.db_conn, product).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("update products success")),
            data: None,
        }),
        Err(err) => Err(Fail {
            error: Some(err.to_string()),
            message: None,
        }),
    }
}
