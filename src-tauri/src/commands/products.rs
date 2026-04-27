use tauri::State;

use tenant_service::products::{
    ListProductsArgs, NewProduct, Product, ProductSearch, ProductsResponse, ProductsService,
};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_products(
    state: State<'_, AppState>,
    args: ListProductsArgs,
) -> SResult<ProductsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ProductsService::list_products(&db_conn, args).await {
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
pub async fn search_products(
    state: State<'_, AppState>,
    search: String,
) -> SResult<Vec<ProductSearch>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ProductsService::search_products(&db_conn, search).await {
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
pub async fn create_product(state: State<'_, AppState>, product: NewProduct) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ProductsService::create_product(&db_conn, product).await {
        Ok(id) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("product created successfully")),
            data: Some(id),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_product(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ProductsService::delete_product(&db_conn, id).await {
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
pub async fn update_product(state: State<'_, AppState>, product: Product) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match ProductsService::update_product(&db_conn, product).await {
        Ok(_) => Ok(Success::<String> {
            error: None,
            message: Option::Some(String::from("update products success")),
            data: None,
        }),
        Err(err) => Err(err.into()),
    }
}
