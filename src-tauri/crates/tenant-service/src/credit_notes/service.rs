use super::dto::*;
use chrono::Utc;
use sea_orm::{
    DatabaseConnection as DbConn, ActiveValue, TransactionError, DbErr, EntityTrait,
    TransactionTrait, ActiveModelTrait,
};
use tenant_entity::{
    credit_note_items::{self, ActiveModel as CreditNoteItemActiveModel},
    credit_notes::{self, ActiveModel as CreditNoteActiveModel},
    invoices,
    prelude::*,
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

                let identifier = format!(
                    "CN-{}-{:03}",
                    Utc::now().format("%y-%m"),
                    Utc::now().timestamp() % 1000
                );

                let cn = CreditNoteActiveModel {
                    invoice_id: ActiveValue::Set(credit_note.invoice_id),
                    client_id: ActiveValue::Set(invoice.client_id),
                    identifier: ActiveValue::Set(Some(identifier.clone())),
                    reason: ActiveValue::Set(credit_note.reason),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let mut total = 0.0f64;
                let mut items = Vec::<CreditNoteItemActiveModel>::new();

                for item in credit_note.items {
                    total += (item.quantity as f64) * item.price;
                    items.push(CreditNoteItemActiveModel {
                        credit_note_id: ActiveValue::Set(cn.id.clone()),
                        product_id: ActiveValue::Set(item.product_id),
                        quantity: ActiveValue::Set(item.quantity),
                        price: ActiveValue::Set(item.price),
                        ..Default::default()
                    });
                }

                if !items.is_empty() {
                    CreditNoteItems::insert_many(items).exec(txn).await?;
                }

                Ok(CreditNoteResponse {
                    id: cn.id,
                    invoice_id: cn.invoice_id,
                    client_id: cn.client_id,
                    identifier: cn.identifier,
                    reason: cn.reason,
                    created_at: cn.created_at.to_string(),
                    total,
                })
            })
        })
        .await
    }
}
