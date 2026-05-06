use super::dto::*;
use crate::status::InvoiceStatus;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection as DbConn, DbErr, EntityTrait,
    QueryFilter, QueryOrder, TransactionError, TransactionTrait,
};
use std::collections::HashMap;
use tenant_entity::{
    credit_note_items::ActiveModel as CreditNoteItemActiveModel,
    credit_notes::ActiveModel as CreditNoteActiveModel,
    inventory_transactions::ActiveModel as InventoryTransactionActiveModel, prelude::*,
};

pub struct CreditNotesService;

impl CreditNotesService {
    pub async fn create_credit_note(
        db: &DbConn,
        credit_note: CreateCreditNote,
    ) -> Result<CreditNoteResponse, TransactionError<DbErr>> {
        db.transaction::<_, CreditNoteResponse, DbErr>(|txn| {
            Box::pin(async move {
                let invoice = Invoices::find_by_id(&credit_note.invoice_id)
                    .one(txn)
                    .await?
                    .ok_or_else(|| DbErr::RecordNotFound("invoice not found".to_string()))?;

                let invoice_status = InvoiceStatus::from_str(&invoice.status).ok_or_else(|| {
                    DbErr::Custom(format!("corrupted invoice status: {}", invoice.status))
                })?;

                if invoice_status != InvoiceStatus::Finalized {
                    return Err(DbErr::Custom(
                        "credit notes can only be created for finalized invoices".to_string(),
                    ));
                }

                if credit_note.items.is_empty() {
                    return Err(DbErr::Custom(
                        "credit note must contain at least one item".to_string(),
                    ));
                }
                let client = Clients::find_by_id(&invoice.client_id)
                    .one(txn)
                    .await?
                    .ok_or_else(|| {
                        DbErr::RecordNotFound("client for credit note not found".to_string())
                    })?;

                let invoice_items = InvoiceItems::find()
                    .filter(tenant_entity::invoice_items::Column::InvoiceId.eq(invoice.id.clone()))
                    .all(txn)
                    .await?;
                let invoice_items_by_product: HashMap<String, tenant_entity::invoice_items::Model> =
                    invoice_items
                        .into_iter()
                        .map(|item| (item.product_id.clone(), item))
                        .collect();

                let cn = CreditNoteActiveModel {
                    invoice_id: ActiveValue::Set(credit_note.invoice_id),
                    client_id: ActiveValue::Set(invoice.client_id),
                    reason: ActiveValue::Set(credit_note.reason),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let cn = CreditNotes::find_by_id(&cn.id)
                    .one(txn)
                    .await?
                    .ok_or_else(|| {
                        DbErr::RecordNotFound("credit note not found after insert".to_string())
                    })?;

                let mut total = 0.0f64;
                let mut items = Vec::<CreditNoteItemActiveModel>::new();

                for item in credit_note.items {
                    if item.quantity <= 0.0 {
                        return Err(DbErr::Custom(
                            "credit note item quantity must be greater than zero".to_string(),
                        ));
                    }

                    if item.price < 0.0 {
                        return Err(DbErr::Custom(
                            "credit note item price cannot be negative".to_string(),
                        ));
                    }

                    let invoice_item =
                        invoice_items_by_product
                            .get(&item.product_id)
                            .ok_or_else(|| {
                                DbErr::Custom(
                                    "credit note item does not belong to the invoice".to_string(),
                                )
                            })?;

                    if item.quantity as f64 > invoice_item.quantity {
                        return Err(DbErr::Custom(
                            "credit note item quantity exceeds invoice quantity".to_string(),
                        ));
                    }

                    if item.price > invoice_item.price {
                        return Err(DbErr::Custom(
                            "credit note item price exceeds invoice price".to_string(),
                        ));
                    }

                    total += (item.quantity as f64) * item.price;
                    items.push(CreditNoteItemActiveModel {
                        credit_note_id: ActiveValue::Set(cn.id.clone()),
                        product_id: ActiveValue::Set(item.product_id.clone()),
                        quantity: ActiveValue::Set(item.quantity),
                        price: ActiveValue::Set(item.price),
                        ..Default::default()
                    });
                    InventoryTransactionActiveModel {
                        product_id: ActiveValue::Set(item.product_id),
                        quantity: ActiveValue::Set(item.quantity),
                        transaction_type: ActiveValue::Set("IN".to_string()),
                        source_type: ActiveValue::Set("CREDIT_NOTE".to_string()),
                        source_id: ActiveValue::Set(Some(cn.id.clone())),
                        unit_price: ActiveValue::Set(Some(item.price as f32)),
                        ..Default::default()
                    }
                    .insert(txn)
                    .await?;
                }

                if !items.is_empty() {
                    CreditNoteItems::insert_many(items).exec(txn).await?;
                }

                Ok(CreditNoteResponse {
                    id: cn.id,
                    invoice_id: cn.invoice_id,
                    invoice_identifier: invoice.identifier,
                    client_id: cn.client_id,
                    full_name: client.full_name,
                    identifier: cn.identifier,
                    reason: cn.reason,
                    created_at: cn.created_at.to_string(),
                    total,
                })
            })
        })
        .await
    }

    pub async fn list_credit_notes(
        db: &DbConn,
        args: ListCreditNotesArgs,
    ) -> Result<CreditNotesListResponse, DbErr> {
        let notes = CreditNotes::find()
            .filter(tenant_entity::credit_notes::Column::IsDeleted.eq(false))
            .order_by_desc(tenant_entity::credit_notes::Column::CreatedAt)
            .all(db)
            .await?;

        let mut response_notes = Vec::new();
        for note in notes {
            let invoice = Invoices::find_by_id(&note.invoice_id)
                .one(db)
                .await?
                .ok_or_else(|| {
                    DbErr::RecordNotFound("invoice for credit note not found".to_string())
                })?;
            let client = Clients::find_by_id(&note.client_id)
                .one(db)
                .await?
                .ok_or_else(|| {
                    DbErr::RecordNotFound("client for credit note not found".to_string())
                })?;
            let items = CreditNoteItems::find()
                .filter(tenant_entity::credit_note_items::Column::CreditNoteId.eq(note.id.clone()))
                .all(db)
                .await?;

            let item_total: f64 = items
                .iter()
                .map(|item| (item.quantity as f64) * item.price)
                .sum();

            response_notes.push(CreditNoteResponse {
                id: note.id,
                invoice_id: note.invoice_id,
                invoice_identifier: invoice.identifier,
                client_id: note.client_id,
                full_name: client.full_name,
                identifier: note.identifier,
                reason: note.reason,
                created_at: note.created_at.to_string(),
                total: item_total,
            });
        }
        let search = args.search.trim().to_lowercase();
        if !search.is_empty() {
            response_notes.retain(|note| {
                [
                    note.identifier.as_deref(),
                    Some(note.full_name.as_str()),
                    note.invoice_identifier.as_deref(),
                    note.reason.as_deref(),
                ]
                .iter()
                .flatten()
                .any(|value| value.to_lowercase().contains(&search))
            });
        }

        if let Some(created_from) = &args.created_from {
            response_notes.retain(|note| note.created_at.as_str() >= created_from.as_str());
        }

        if let Some(created_to) = &args.created_to {
            response_notes.retain(|note| note.created_at.as_str() <= created_to.as_str());
        }

        if let Some(total_min) = args.total_min {
            response_notes.retain(|note| note.total >= total_min);
        }

        if let Some(total_max) = args.total_max {
            response_notes.retain(|note| note.total <= total_max);
        }

        match args.sort.as_deref() {
            Some("identifier") => response_notes.sort_by(|a, b| a.identifier.cmp(&b.identifier)),
            Some("full_name") => response_notes.sort_by(|a, b| a.full_name.cmp(&b.full_name)),
            Some("invoice_identifier") => {
                response_notes.sort_by(|a, b| a.invoice_identifier.cmp(&b.invoice_identifier))
            }
            Some("reason") => response_notes.sort_by(|a, b| a.reason.cmp(&b.reason)),
            Some("total") => response_notes.sort_by(|a, b| {
                a.total
                    .partial_cmp(&b.total)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            Some("created_at") => response_notes.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
            _ => {}
        }

        if matches!(args.direction.as_deref(), Some("desc")) {
            response_notes.reverse();
        }

        let total = response_notes.len() as i64;
        let limit = args.limit.clamp(1, 100) as usize;
        let offset = args.offset.max(0) as usize;
        let response_notes = response_notes
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(CreditNotesListResponse {
            count: total,
            notes: response_notes,
        })
    }

    pub async fn get_credit_note(
        db: &DbConn,
        id: String,
    ) -> Result<CreditNoteDetailsResponse, DbErr> {
        let note = CreditNotes::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("credit note not found".to_string()))?;
        let invoice = Invoices::find_by_id(&note.invoice_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                DbErr::RecordNotFound("invoice for credit note not found".to_string())
            })?;
        let client = Clients::find_by_id(&note.client_id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("client for credit note not found".to_string()))?;

        let items = CreditNoteItems::find()
            .filter(tenant_entity::credit_note_items::Column::CreditNoteId.eq(note.id.clone()))
            .all(db)
            .await?;

        let total: f64 = items
            .iter()
            .map(|item| (item.quantity as f64) * item.price)
            .sum();
        let mut response_items = Vec::new();

        for item in items {
            let product = Products::find_by_id(&item.product_id)
                .one(db)
                .await?
                .ok_or_else(|| {
                    DbErr::RecordNotFound("product for credit note not found".to_string())
                })?;
            response_items.push(CreditNoteProductItem {
                product_id: item.product_id,
                name: product.name,
                quantity: item.quantity,
                price: item.price,
            });
        }

        Ok(CreditNoteDetailsResponse {
            id: note.id,
            invoice_id: note.invoice_id,
            invoice_identifier: invoice.identifier,
            client_id: note.client_id,
            identifier: note.identifier,
            reason: note.reason,
            created_at: note.created_at.to_string(),
            total,
            client: CreditNoteClientInfo {
                full_name: client.full_name,
                email: client.email,
                address: client.address,
                phone_number: client.phone_number,
                ice: client.ice,
                if_number: client.if_number,
                rc: client.rc,
                patente: client.patente,
            },
            items: response_items,
        })
    }
}
