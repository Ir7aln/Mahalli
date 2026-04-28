use tauri::State;

use tenant_service::invoices::{
    AddInvoicePayment, InvoiceDetailsResponse, InvoiceProductItem, InvoiceWithClient,
    InvoicesResponse, InvoicesService, ListInvoicesArgs, NewInvoice, UpdateInvoice,
    UpdateInvoiceStatus,
};

use crate::AppState;

use super::{tenant_db_or_fail, SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn create_invoice_from_order(state: State<'_, AppState>, id: String) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::create_invoice_from_order(&db_conn, id).await {
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
pub async fn create_invoice_from_delivery_note(
    state: State<'_, AppState>,
    id: String,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::create_invoice_from_delivery_note(&db_conn, id).await {
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
pub async fn list_invoices(
    state: State<'_, AppState>,
    args: ListInvoicesArgs,
) -> SResult<InvoicesResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::list_invoices(&db_conn, args).await {
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
pub async fn list_invoice_products(
    state: State<'_, AppState>,
    id: String,
) -> SResult<Vec<InvoiceProductItem>> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::list_invoice_products(&db_conn, id).await {
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
pub async fn create_invoice(state: State<'_, AppState>, invoice: NewInvoice) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::create_invoice(&db_conn, invoice).await {
        Ok(id) => Ok(Success {
            error: None,
            message: None,
            data: Some(id),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_invoice(state: State<'_, AppState>, invoice: UpdateInvoice) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::update_invoice(&db_conn, invoice).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update invoices success")),
            data: None,
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn add_invoice_payment(
    state: State<'_, AppState>,
    payment: AddInvoicePayment,
) -> SResult<String> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::add_invoice_payment(&db_conn, payment).await {
        Ok(id) => Ok(Success {
            error: None,
            message: Option::Some(String::from("invoice payment added successfully")),
            data: Some(id),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_invoice_status(
    state: State<'_, AppState>,
    invoice: UpdateInvoiceStatus,
) -> SResult<()> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::update_invoice_status(&db_conn, invoice).await {
        Ok(_) => Ok(Success {
            error: None,
            message: Option::Some(String::from("update invoices success")),
            data: None,
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_invoice(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::delete_invoice(&db_conn, id).await {
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
pub async fn get_invoice(state: State<'_, AppState>, id: String) -> SResult<InvoiceWithClient> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::get_invoice(&db_conn, id).await {
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
pub async fn get_invoice_details(
    state: State<'_, AppState>,
    id: String,
) -> SResult<InvoiceDetailsResponse> {
    let db_conn = tenant_db_or_fail(&state).await?;
    match InvoicesService::get_invoice_details(&db_conn, id).await {
        Ok(res) => Ok(Success {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => Err(err.into()),
    }
}
