use sea_orm::entity::prelude::Decimal;
use sea_orm::*;

use crate::{entities::*, models::*};

type TxnRes<T> = Result<T, TransactionError<DbErr>>;

pub struct TransactionService;

impl TransactionService {
    pub async fn create_quote(db: &DbConn, quote: NewQuote) -> TxnRes<String> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                let created_quote = QuoteActiveModel {
                    client_id: ActiveValue::Set(quote.client_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let mut items = Vec::<QuoteItemActiveModel>::new();

                for item in quote.items {
                    items.push(QuoteItemActiveModel {
                        quote_id: ActiveValue::Set(created_quote.id.clone()),
                        price: ActiveValue::Set(item.price),
                        quantity: ActiveValue::Set(item.quantity),
                        product_id: ActiveValue::Set(item.product_id),
                        ..Default::default()
                    })
                }
                if items.len() > 0 {
                    QuoteItems::insert_many(items).exec(txn).await?;
                }

                Ok(created_quote.id)
            })
        })
        .await
    }
    pub async fn update_quote(db: &DbConn, quote: UpdateQuote) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let quote_model = Quotes::find_by_id(quote.id.clone()).one(txn).await?;
                let mut quote_active: QuoteActiveModel = quote_model.unwrap().into();
                quote_active.client_id = ActiveValue::Set(quote.client_id);
                quote_active.update(txn).await?;

                let mut items = Vec::<QuoteItemActiveModel>::new();

                for item in quote.items {
                    match item.id {
                        Some(id) => {
                            let quote_item_model = QuoteItems::find_by_id(&id).one(txn).await?;
                            let mut quote_item_active: QuoteItemActiveModel =
                                quote_item_model.unwrap().into();
                            quote_item_active.product_id = ActiveValue::Set(item.product_id);
                            quote_item_active.price = ActiveValue::Set(item.price);
                            quote_item_active.quantity = ActiveValue::Set(item.quantity);
                            quote_item_active.update(txn).await?;
                        }
                        None => items.push(QuoteItemActiveModel {
                            quote_id: ActiveValue::Set(quote.id.clone()),
                            price: ActiveValue::Set(item.price),
                            quantity: ActiveValue::Set(item.quantity),
                            product_id: ActiveValue::Set(item.product_id),
                            ..Default::default()
                        }),
                    }
                }
                if items.len() > 0 {
                    QuoteItems::insert_many(items).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
    }
    pub async fn create_order(db: &DbConn, order: NewOrder) -> TxnRes<String> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                let created_order = OrderActiveModel {
                    client_id: ActiveValue::Set(order.client_id),
                    status: ActiveValue::Set(order.status),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let mut items = Vec::<OrderItemActiveModel>::new();

                for item in order.items {
                    let created_inventory = InventoryActiveModel {
                        product_id: ActiveValue::Set(item.product_id),
                        quantity: ActiveValue::Set(item.quantity),
                        transaction_type: ActiveValue::Set("OUT".to_string()),
                        ..Default::default()
                    }
                    .insert(txn)
                    .await?;

                    items.push(OrderItemActiveModel {
                        order_id: ActiveValue::Set(created_order.id.clone()),
                        price: ActiveValue::Set(item.price),
                        inventory_id: ActiveValue::Set(created_inventory.id),
                        ..Default::default()
                    })
                }
                if items.len() > 0 {
                    OrderItems::insert_many(items).exec(txn).await?;
                }

                Ok(created_order.id)
            })
        })
        .await
    }
    pub async fn update_order(db: &DbConn, order: UpdateOrder) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let order_model = Orders::find_by_id(order.id.clone()).one(txn).await?;
                let mut order_active: OrderActiveModel = order_model.unwrap().into();
                order_active.client_id = ActiveValue::Set(order.client_id);
                order_active.status = ActiveValue::Set(order.status);
                order_active.save(txn).await?;

                let mut items = Vec::<OrderItemActiveModel>::new();

                for item in order.items {
                    match item.id {
                        Some(id) => {
                            let order_item_model = OrderItems::find_by_id(id).one(txn).await?;
                            let mut order_item_active: OrderItemActiveModel =
                                order_item_model.unwrap().into();
                            order_item_active.price = ActiveValue::Set(item.price);
                            order_item_active.update(txn).await?;
                            let inventory_model =
                                InventoryTransactions::find_by_id(item.inventory_id.unwrap())
                                    .one(txn)
                                    .await?;
                            let mut inventory_active: InventoryActiveModel =
                                inventory_model.unwrap().into();
                            inventory_active.quantity = ActiveValue::Set(item.quantity);
                            inventory_active.product_id = ActiveValue::Set(item.product_id);
                            inventory_active.update(txn).await?;
                        }
                        None => {
                            let created_inventory = InventoryActiveModel {
                                product_id: ActiveValue::Set(item.product_id),
                                quantity: ActiveValue::Set(item.quantity),
                                transaction_type: ActiveValue::Set("OUT".to_string()),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            items.push(OrderItemActiveModel {
                                order_id: ActiveValue::Set(order.id.clone()),
                                price: ActiveValue::Set(item.price),
                                inventory_id: ActiveValue::Set(created_inventory.id),
                                ..Default::default()
                            })
                        }
                    }
                }
                if items.len() > 0 {
                    OrderItems::insert_many(items).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
    }
    pub async fn create_invoice(db: &DbConn, invoice: NewInvoice) -> TxnRes<String> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                let created_order = OrderActiveModel {
                    client_id: ActiveValue::Set(invoice.client_id.clone()),
                    status: ActiveValue::Set("PENDING".to_string()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let created_invoice = InvoiceActiveModel {
                    client_id: ActiveValue::Set(invoice.client_id),
                    status: ActiveValue::Set(invoice.status),
                    paid_amount: ActiveValue::Set(invoice.paid_amount),
                    order_id: ActiveValue::Set(created_order.id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                // Invoices have their own item snapshot; do not share order_items rows with orders.
                let mut invoice_items = Vec::<InvoiceItemActiveModel>::new();
                for item in invoice.items {
                    let created_inventory = InventoryActiveModel {
                        product_id: ActiveValue::Set(item.product_id.clone()),
                        quantity: ActiveValue::Set(item.quantity),
                        transaction_type: ActiveValue::Set("OUT".to_string()),
                        ..Default::default()
                    }
                    .insert(txn)
                    .await?;

                    invoice_items.push(InvoiceItemActiveModel {
                        invoice_id: ActiveValue::Set(created_invoice.id.clone()),
                        product_id: ActiveValue::Set(item.product_id),
                        price: ActiveValue::Set(item.price),
                        quantity: ActiveValue::Set(item.quantity),
                        inventory_id: ActiveValue::Set(Some(created_inventory.id)),
                        ..Default::default()
                    });
                }
                if !invoice_items.is_empty() {
                    InvoiceItems::insert_many(invoice_items).exec(txn).await?;
                }

                Ok(created_invoice.id)
            })
        })
        .await
    }
    pub async fn update_invoice(db: &DbConn, invoice: UpdateInvoice) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let invoice_model = Invoices::find_by_id(invoice.id.clone()).one(txn).await?;
                let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
                invoice_active.client_id = ActiveValue::Set(invoice.client_id);
                invoice_active.status = ActiveValue::Set(invoice.status);
                invoice_active.paid_amount = ActiveValue::Set(invoice.paid_amount);
                invoice_active.save(txn).await?;

                // Update invoice items only; invoices must not mutate orders.
                let mut new_items = Vec::<InvoiceItemActiveModel>::new();

                for item in invoice.items {
                    match item.id {
                        Some(id) => {
                            let item_model = InvoiceItems::find_by_id(id).one(txn).await?;
                            let mut item_active: InvoiceItemActiveModel =
                                item_model.unwrap().into();
                            item_active.price = ActiveValue::Set(item.price);
                            item_active.quantity = ActiveValue::Set(item.quantity);
                            item_active.product_id = ActiveValue::Set(item.product_id);
                            // Keep the original inventory_id snapshot unless client explicitly sends one.
                            if let Some(inv_id) = item.inventory_id {
                                item_active.inventory_id = ActiveValue::Set(Some(inv_id));
                            }
                            item_active.save(txn).await?;
                        }
                        None => {
                            // New invoice item: create a stock OUT transaction and reference it from the invoice item.
                            let created_inventory = InventoryActiveModel {
                                product_id: ActiveValue::Set(item.product_id.clone()),
                                quantity: ActiveValue::Set(item.quantity),
                                transaction_type: ActiveValue::Set("OUT".to_string()),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            new_items.push(InvoiceItemActiveModel {
                                invoice_id: ActiveValue::Set(invoice.id.clone()),
                                product_id: ActiveValue::Set(item.product_id),
                                price: ActiveValue::Set(item.price),
                                quantity: ActiveValue::Set(item.quantity),
                                inventory_id: ActiveValue::Set(Some(created_inventory.id)),
                                ..Default::default()
                            });
                        }
                    }
                }
                if !new_items.is_empty() {
                    InvoiceItems::insert_many(new_items).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
    }
    pub async fn create_order_from_quote(db: &DbConn, id: String) -> TxnRes<String> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                match Orders::find()
                    .filter(orders::Column::QuoteId.eq(&id))
                    .one(txn)
                    .await?
                {
                    Some(order) => Ok(order.id),
                    None => match Quotes::find_by_id(&id).one(txn).await? {
                        Some(quote) => {
                            let order = OrderActiveModel {
                                client_id: ActiveValue::Set(quote.client_id),
                                status: ActiveValue::Set("PENDING".to_string()),
                                quote_id: ActiveValue::Set(Some(quote.id)),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            let quote_items = QuoteItems::find()
                                .filter(quote_items::Column::QuoteId.eq(id))
                                .all(txn)
                                .await?;

                            let mut items = Vec::<OrderItemActiveModel>::new();

                            for item in quote_items {
                                let created_inventory = InventoryActiveModel {
                                    product_id: ActiveValue::Set(item.product_id),
                                    quantity: ActiveValue::Set(item.quantity),
                                    transaction_type: ActiveValue::Set("OUT".to_string()),
                                    ..Default::default()
                                }
                                .insert(txn)
                                .await?;

                                items.push(OrderItemActiveModel {
                                    order_id: ActiveValue::Set(order.id.clone()),
                                    price: ActiveValue::Set(item.price),
                                    inventory_id: ActiveValue::Set(created_inventory.id),
                                    ..Default::default()
                                })
                            }
                            if items.len() > 0 {
                                OrderItems::insert_many(items).exec(txn).await?;
                            }
                            Ok(order.id)
                        }
                        None => Err(DbErr::RecordNotFound("no quote".to_string())),
                    },
                }
            })
        })
        .await
    }
    pub async fn create_invoice_from_order(db: &DbConn, id: String) -> TxnRes<String> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                match Invoices::find()
                    .filter(invoices::Column::OrderId.eq(&id))
                    .one(txn)
                    .await?
                {
                    Some(invoice) => Ok(invoice.id),
                    None => match Orders::find_by_id(&id).one(txn).await? {
                        Some(order) => {
                            let status = "DRAFT".to_string();
                            let invoice = InvoiceActiveModel {
                                client_id: ActiveValue::Set(order.client_id),
                                paid_amount: ActiveValue::Set(Decimal::ZERO),
                                status: ActiveValue::Set(status),
                                order_id: ActiveValue::Set(order.id),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            // Snapshot order items into invoice_items so invoice edits don't affect the order.
                            let order_items = OrderItems::find()
                                .filter(order_items::Column::OrderId.eq(id))
                                .all(txn)
                                .await?;

                            let mut invoice_items = Vec::<InvoiceItemActiveModel>::new();
                            for item in order_items {
                                let inv_txn = InventoryTransactions::find_by_id(&item.inventory_id)
                                    .one(txn)
                                    .await?
                                    .ok_or_else(|| {
                                        DbErr::RecordNotFound(format!(
                                            "missing inventory_transaction {}",
                                            item.inventory_id
                                        ))
                                    })?;

                                invoice_items.push(InvoiceItemActiveModel {
                                    invoice_id: ActiveValue::Set(invoice.id.clone()),
                                    product_id: ActiveValue::Set(inv_txn.product_id),
                                    price: ActiveValue::Set(item.price),
                                    quantity: ActiveValue::Set(inv_txn.quantity),
                                    inventory_id: ActiveValue::Set(Some(item.inventory_id)),
                                    ..Default::default()
                                });
                            }
                            if !invoice_items.is_empty() {
                                InvoiceItems::insert_many(invoice_items).exec(txn).await?;
                            }
                            Ok(invoice.id)
                        }
                        None => Err(DbErr::RecordNotFound("no order".to_string())),
                    },
                }
            })
        })
        .await
    }
}
