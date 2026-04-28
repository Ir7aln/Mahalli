use serde::{Deserialize, Serialize};
use specta::Type;
use tenant_service::sea_orm::DatabaseConnection as TenantDatabaseConnection;
use tenant_service::sea_orm::{DbErr, TransactionError};

pub mod clients;
pub mod column_preferences;
pub mod credit_notes;
pub mod dashboard;
pub mod databases;
pub mod delivery_notes;
pub mod inventory;
pub mod invoice_items;
pub mod invoices;
pub mod order_items;
pub mod orders;
pub mod products;
pub mod quote_items;
pub mod quotes;
pub mod seed;
pub mod seller_profile;
pub mod templates;

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Success<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Fail {
    pub code: Option<String>,
    pub i18n_key: Option<String>,
    pub error: Option<String>,
    pub message: Option<String>,
}

pub type SResult<T> = Result<Success<T>, Fail>;

impl Fail {
    fn from_raw_error(raw: String) -> Self {
        let (code, i18n_key, message) = map_error_details(&raw);
        Self {
            code: Some(code.to_string()),
            i18n_key: Some(i18n_key.to_string()),
            error: Some(raw),
            message: Some(message),
        }
    }
}

impl From<String> for Fail {
    fn from(raw: String) -> Self {
        Self::from_raw_error(raw)
    }
}

impl From<DbErr> for Fail {
    fn from(err: DbErr) -> Self {
        Self::from_raw_error(err.to_string())
    }
}

impl From<TransactionError<DbErr>> for Fail {
    fn from(err: TransactionError<DbErr>) -> Self {
        match err {
            TransactionError::Connection(db_err) | TransactionError::Transaction(db_err) => {
                Self::from(db_err)
            }
        }
    }
}

fn map_error_details(raw: &str) -> (&'static str, &'static str, String) {
    let normalized = raw.to_ascii_lowercase();

    if normalized.contains("no active tenant database selected") {
        return (
            "DATABASE_REQUIRED",
            "notifications.errors.database-required",
            String::from("Select a database before using the app."),
        );
    }

    if normalized.contains("source database not found") {
        return (
            "DATABASE_SOURCE_NOT_FOUND",
            "notifications.errors.database-source-not-found",
            String::from("The source database could not be found."),
        );
    }

    if normalized.contains("source database file does not exist") {
        return (
            "DATABASE_SOURCE_FILE_MISSING",
            "notifications.errors.database-source-file-missing",
            String::from("The source database file is missing."),
        );
    }

    if normalized.contains("target database file already exists") {
        return (
            "DATABASE_FILE_ALREADY_EXISTS",
            "notifications.errors.database-file-already-exists",
            String::from("A database file with the same name already exists."),
        );
    }

    if normalized.contains("no active database found")
        || normalized.contains("active database not found after switch")
    {
        return (
            "DATABASE_NOT_FOUND",
            "notifications.errors.database-not-found",
            String::from("The selected database could not be found."),
        );
    }

    if normalized.contains("unable to open database file") {
        return (
            "DATABASE_OPEN_FAILED",
            "notifications.errors.database-open-failed",
            String::from("The database file could not be opened."),
        );
    }

    if normalized.contains("created database was not found") {
        return (
            "DATABASE_CREATE_INCOMPLETE",
            "notifications.errors.database-create-incomplete",
            String::from("The database was created, but it could not be loaded afterwards."),
        );
    }

    if normalized.contains("no client") {
        return (
            "CLIENT_NOT_FOUND",
            "notifications.errors.client-not-found",
            String::from("The client could not be found."),
        );
    }

    if normalized.contains("no quote") {
        return (
            "QUOTE_NOT_FOUND",
            "notifications.errors.quote-not-found",
            String::from("The quote could not be found."),
        );
    }

    if normalized.contains("delivery note source order not found") {
        return (
            "DELIVERY_NOTE_ORDER_NOT_FOUND",
            "notifications.errors.delivery-note-order-not-found",
            String::from("The source order for this delivery note could not be found."),
        );
    }

    if normalized.contains("no order") || normalized.contains("order not found") {
        return (
            "ORDER_NOT_FOUND",
            "notifications.errors.order-not-found",
            String::from("The order could not be found."),
        );
    }

    if normalized.contains("delivery note not found") {
        return (
            "DELIVERY_NOTE_NOT_FOUND",
            "notifications.errors.delivery-note-not-found",
            String::from("The delivery note could not be found."),
        );
    }

    if normalized.contains("credit note") && normalized.contains("not found") {
        return (
            "CREDIT_NOTE_NOT_FOUND",
            "notifications.errors.credit-note-not-found",
            String::from("The credit note could not be found."),
        );
    }

    if normalized.contains("delivery note inventory transaction missing") {
        return (
            "DELIVERY_NOTE_INVENTORY_MISSING",
            "notifications.errors.delivery-note-inventory-missing",
            String::from("A linked stock movement for this delivery note could not be found."),
        );
    }

    if normalized.contains("no invoice") || normalized.contains("invoice not found") {
        if normalized.contains("credit note") {
            return (
                "CREDIT_NOTE_INVOICE_NOT_FOUND",
                "notifications.errors.credit-note-invoice-not-found",
                String::from("The invoice for this credit note could not be found."),
            );
        }
        return (
            "INVOICE_NOT_FOUND",
            "notifications.errors.invoice-not-found",
            String::from("The invoice could not be found."),
        );
    }

    if normalized.contains("invalid payment date") {
        return (
            "INVOICE_PAYMENT_DATE_INVALID",
            "notifications.errors.invoice-payment-date-invalid",
            String::from("The payment date is invalid."),
        );
    }

    if normalized.contains("payment amount must be greater than zero") {
        return (
            "INVOICE_PAYMENT_AMOUNT_INVALID",
            "notifications.errors.invoice-payment-amount-invalid",
            String::from("The payment amount must be greater than zero."),
        );
    }

    if normalized.contains("cannot add payment to a deleted invoice") {
        return (
            "INVOICE_PAYMENT_DELETED",
            "notifications.errors.invoice-payment-deleted",
            String::from("Payments cannot be added to a deleted invoice."),
        );
    }

    if normalized.contains("payment amount exceeds unpaid amount") {
        return (
            "INVOICE_PAYMENT_EXCEEDS_REMAINING",
            "notifications.errors.invoice-payment-exceeds-remaining",
            String::from("The payment amount exceeds the remaining unpaid amount."),
        );
    }

    if normalized.contains("only paid invoices can be finalized") {
        return (
            "INVOICE_FINALIZE_REQUIRES_PAID",
            "notifications.errors.invoice-finalize-requires-paid",
            String::from("Only fully paid invoices can be finalized."),
        );
    }

    if normalized.contains("finalized invoices cannot be edited") {
        return (
            "INVOICE_FINALIZED_EDIT_BLOCKED",
            "notifications.errors.invoice-finalized-edit-blocked",
            String::from("Finalized invoices cannot be edited."),
        );
    }

    if normalized.contains("finalized invoices cannot be deleted") {
        return (
            "INVOICE_FINALIZED_DELETE_BLOCKED",
            "notifications.errors.invoice-finalized-delete-blocked",
            String::from("Finalized invoices cannot be deleted."),
        );
    }

    if normalized.contains("credit notes can only be created for finalized invoices") {
        return (
            "CREDIT_NOTE_REQUIRES_FINALIZED_INVOICE",
            "notifications.errors.credit-note-requires-finalized-invoice",
            String::from("Credit notes can only be created for finalized invoices."),
        );
    }

    if normalized.contains("credit note must contain at least one item") {
        return (
            "CREDIT_NOTE_EMPTY",
            "notifications.errors.credit-note-empty",
            String::from("A credit note must contain at least one item."),
        );
    }

    if normalized.contains("credit note item quantity must be greater than zero") {
        return (
            "CREDIT_NOTE_QUANTITY_INVALID",
            "notifications.errors.credit-note-quantity-invalid",
            String::from("Credit note quantities must be greater than zero."),
        );
    }

    if normalized.contains("credit note item price cannot be negative") {
        return (
            "CREDIT_NOTE_PRICE_INVALID",
            "notifications.errors.credit-note-price-invalid",
            String::from("Credit note prices cannot be negative."),
        );
    }

    if normalized.contains("credit note item does not belong to the invoice") {
        return (
            "CREDIT_NOTE_ITEM_NOT_ON_INVOICE",
            "notifications.errors.credit-note-item-not-on-invoice",
            String::from("One credit note item is not part of the source invoice."),
        );
    }

    if normalized.contains("credit note item quantity exceeds invoice quantity") {
        return (
            "CREDIT_NOTE_QUANTITY_EXCEEDS_INVOICE",
            "notifications.errors.credit-note-quantity-exceeds-invoice",
            String::from("A credit note quantity exceeds the invoiced quantity."),
        );
    }

    if normalized.contains("credit note item price exceeds invoice price") {
        return (
            "CREDIT_NOTE_PRICE_EXCEEDS_INVOICE",
            "notifications.errors.credit-note-price-exceeds-invoice",
            String::from("A credit note price exceeds the invoiced price."),
        );
    }

    if normalized.contains("invalid order status") || normalized.contains("corrupted order status")
    {
        return (
            "ORDER_STATUS_INVALID",
            "notifications.errors.order-status-invalid",
            String::from("The order status is invalid."),
        );
    }

    if normalized.contains("invalid invoice status")
        || normalized.contains("corrupted invoice status")
    {
        return (
            "INVOICE_STATUS_INVALID",
            "notifications.errors.invoice-status-invalid",
            String::from("The invoice status is invalid."),
        );
    }

    if normalized.contains("invalid status transition from") {
        return (
            "STATUS_TRANSITION_INVALID",
            "notifications.errors.status-transition-invalid",
            String::from("That status change is not allowed."),
        );
    }

    if normalized.contains("missing inventory_transaction") {
        return (
            "INVENTORY_TRANSACTION_MISSING",
            "notifications.errors.inventory-transaction-missing",
            String::from("A linked inventory transaction could not be found."),
        );
    }

    if normalized.contains("no data found") {
        return (
            "DATA_UNAVAILABLE",
            "notifications.errors.data-unavailable",
            String::from("No data is available for this view yet."),
        );
    }

    if normalized.contains("unique constraint failed") {
        return (
            "CONFLICT",
            "notifications.errors.conflict",
            String::from("This action conflicts with existing data."),
        );
    }

    (
        "UNEXPECTED_ERROR",
        "notifications.error.description",
        String::from("Something went wrong. Please try again."),
    )
}

pub async fn tenant_db_or_fail(state: &crate::AppState) -> Result<TenantDatabaseConnection, Fail> {
    state.tenant_db().await.map_err(Fail::from)
}
