use tauri::State;

use tenant_service::{
    InvoiceDetailsResponse, InvoiceProductItem, InvoiceWithClient, InvoicesResponse, ListArgs,
    MutationsService, NewInvoice, QueriesService, TransactionService, UpdateInvoice, UpdateStatus,
};

use crate::AppState;

use super::{tenant_db_or_fail, Fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_invoice_from_order(state: State<'_, AppState>, id: String) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match TransactionService::create_invoice_from_order(&db_conn, id).await {
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
pub async fn list_invoices(
    state: State<'_, AppState>,
    args: ListArgs,
) -> SResult<InvoicesResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::list_invoices(&db_conn, args).await {
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
pub async fn list_invoice_products(
    state: State<'_, AppState>,
    id: String,
) -> SResult<Vec<InvoiceProductItem>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::list_invoice_products(&db_conn, id).await {
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
pub async fn create_invoice(state: State<'_, AppState>, invoice: NewInvoice) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match TransactionService::create_invoice(&db_conn, invoice).await {
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
pub async fn update_invoice(state: State<'_, AppState>, invoice: UpdateInvoice) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match TransactionService::update_invoice(&db_conn, invoice).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update invoices success")),
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
pub async fn update_invoice_status(
    state: State<'_, AppState>,
    invoice: UpdateStatus,
) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::update_invoice_status(&db_conn, invoice).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update invoices success")),
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
pub async fn delete_invoice(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match MutationsService::delete_invoice(&db_conn, id).await {
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
pub async fn get_invoice(state: State<'_, AppState>, id: String) -> SResult<InvoiceWithClient> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::get_invoice(&db_conn, id).await {
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
pub async fn get_invoice_details(
    state: State<'_, AppState>,
    id: String,
) -> SResult<InvoiceDetailsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match QueriesService::get_invoice_details(&db_conn, id).await {
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
