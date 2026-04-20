use tauri::State;

use tenant_service::quotes::{
    ListQuotesArgs, NewQuote, QuoteDetailsResponse, QuoteProductItem, QuoteWithClient,
    QuotesResponse, QuotesService, UpdateQuote,
};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn list_quotes(
    state: State<'_, AppState>,
    args: ListQuotesArgs,
) -> SResult<QuotesResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::list_quotes(&db_conn, args).await {
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
pub async fn list_quote_products(
    state: State<'_, AppState>,
    id: String,
) -> SResult<Vec<QuoteProductItem>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::list_quote_products(&db_conn, id).await {
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
pub async fn create_quote(state: State<'_, AppState>, quote: NewQuote) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::create_quote(&db_conn, quote).await {
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
pub async fn update_quote(state: State<'_, AppState>, quote: UpdateQuote) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::update_quote(&db_conn, quote).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update quotes success")),
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
pub async fn delete_quote(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::delete_quote(&db_conn, id).await {
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
pub async fn get_quote(state: State<'_, AppState>, id: String) -> SResult<QuoteWithClient> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::get_quote(&db_conn, id).await {
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
pub async fn get_quote_details(
    state: State<'_, AppState>,
    id: String,
) -> SResult<QuoteDetailsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QuotesService::get_quote_details(&db_conn, id).await {
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
