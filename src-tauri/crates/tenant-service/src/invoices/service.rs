use super::dto::*;
use crate::InvoiceStatus;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement,
    },
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients::{self, Entity as Clients},
    delivery_note_items::{self, Entity as DeliveryNoteItems},
    delivery_notes::{self, Entity as DeliveryNotes},
    inventory_transactions::{
        ActiveModel as InventoryActiveModel, Entity as InventoryTransactions,
    },
    invoice_items::{self, ActiveModel as InvoiceItemActiveModel, Entity as InvoiceItems},
    invoice_payments::{self, ActiveModel as InvoicePaymentActiveModel, Entity as InvoicePayments},
    invoices::{self, ActiveModel as InvoiceActiveModel, Entity as Invoices},
    order_items::{self, Entity as OrderItems},
    orders::{ActiveModel as OrderActiveModel, Entity as Orders},
    products::{self, Entity as Products},
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn invoice_products_count_expr() -> SimpleExpr {
    SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::count(Expr::col((
                        InvoiceItems,
                        invoice_items::Column::Id,
                    )))),
                    Expr::val(0i64),
                ])))
                .from(InvoiceItems)
                .cond_where(
                    Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                        .equals((Invoices, invoices::Column::Id)),
                )
                .to_owned(),
        )),
    )
}

fn invoice_total_expr() -> SimpleExpr {
    SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ])))
                .from(InvoiceItems)
                .cond_where(
                    Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                        .equals((Invoices, invoices::Column::Id)),
                )
                .to_owned(),
        )),
    )
}

fn invoice_paid_amount_expr() -> SimpleExpr {
    SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::sum(Expr::col((
                        InvoicePayments,
                        invoice_payments::Column::Amount,
                    )))),
                    Expr::val(0.0f64),
                ])))
                .from(InvoicePayments)
                .cond_where(
                    Expr::col((InvoicePayments, invoice_payments::Column::InvoiceId))
                        .equals((Invoices, invoices::Column::Id)),
                )
                .to_owned(),
        )),
    )
}

async fn invoice_total_by_id<C>(db: &C, invoice_id: &str) -> Result<f64, DbErr>
where
    C: ConnectionTrait,
{
    let total = InvoiceItems::find()
        .select_only()
        .expr_as(
            Func::coalesce([
                Expr::expr(Func::sum(
                    Expr::col(invoice_items::Column::Quantity)
                        .mul(Expr::col(invoice_items::Column::Price)),
                )),
                Expr::val(0.0f64),
            ]),
            "total",
        )
        .filter(invoice_items::Column::InvoiceId.eq(invoice_id))
        .into_tuple::<f64>()
        .one(db)
        .await?
        .unwrap_or(0.0);

    Ok(total)
}

async fn invoice_paid_by_id<C>(db: &C, invoice_id: &str) -> Result<f64, DbErr>
where
    C: ConnectionTrait,
{
    let paid = InvoicePayments::find()
        .select_only()
        .expr_as(
            Func::coalesce([
                Expr::expr(Func::sum(Expr::col(invoice_payments::Column::Amount))),
                Expr::val(0.0f64),
            ]),
            "paid_amount",
        )
        .filter(invoice_payments::Column::InvoiceId.eq(invoice_id))
        .into_tuple::<f64>()
        .one(db)
        .await?
        .unwrap_or(0.0);

    Ok(paid)
}

async fn invoice_payments_by_id<C>(
    db: &C,
    invoice_id: &str,
) -> Result<Vec<SelectInvoicePayment>, DbErr>
where
    C: ConnectionTrait,
{
    let payments = InvoicePayments::find()
        .select_only()
        .columns([
            invoice_payments::Column::Id,
            invoice_payments::Column::Description,
            invoice_payments::Column::Amount,
        ])
        .expr_as(
            Expr::col(invoice_payments::Column::PaymentDate),
            "payment_date",
        )
        .filter(invoice_payments::Column::InvoiceId.eq(invoice_id))
        .order_by(invoice_payments::Column::PaymentDate, Order::Desc)
        .into_model::<SelectInvoicePayment>()
        .all(db)
        .await?;

    Ok(payments)
}

fn parse_payment_date(value: &str) -> Result<NaiveDateTime, DbErr> {
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| DbErr::Custom("invalid payment date".to_string()));
    }

    if let Ok(date_time) = DateTime::parse_from_rfc3339(value) {
        return Ok(date_time.naive_utc());
    }

    if let Ok(date_time) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        return Ok(date_time);
    }

    Err(DbErr::Custom("invalid payment date".to_string()))
}

fn normalize_invoice_status(current_status: &str, total: f64, paid_amount: f64) -> Option<String> {
    if current_status == "CANCELLED" || total <= 0.0 {
        return None;
    }

    if paid_amount >= total {
        return Some("PAID".to_string());
    }

    if paid_amount > 0.0 && paid_amount < total {
        return Some("PARTIALLY_PAID".to_string());
    }

    None
}

fn invoice_search_condition(search: &str) -> Cond {
    let pattern = format!("%{}%", search);
    Cond::any()
        .add(Expr::col((Clients, clients::Column::FullName)).like(pattern.clone()))
        .add(Expr::col((Invoices, invoices::Column::Identifier)).like(pattern))
}

pub struct InvoicesService;

impl InvoicesService {
    pub async fn list_invoices(
        db: &DbConn,
        args: ListInvoicesArgs,
    ) -> Result<InvoicesResponse, DbErr> {
        let count = Invoices::find()
            .join(JoinType::Join, invoices::Relation::Clients.def())
            .filter(
                Cond::all()

                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(invoice_search_condition(&args.search)),
            )
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Invoices, invoices::Column::Status)).eq(v))
            })
            .apply_if(args.created_from.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', invoices.created_at) >= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.created_to.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', invoices.created_at) <= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Invoices)
            .exprs([
                Expr::col((Invoices, invoices::Column::Id)),
                Expr::col((Invoices, invoices::Column::Status)),
                Expr::col((Invoices, invoices::Column::Identifier)),
                Expr::col((Invoices, invoices::Column::CreatedAt)),
                Expr::col((Invoices, invoices::Column::ClientId)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::Ice)),
                Expr::col((Clients, clients::Column::IfNumber)),
                Expr::col((Clients, clients::Column::Rc)),
                Expr::col((Clients, clients::Column::Patente)),
            ])
            .expr_as(invoice_products_count_expr(), Alias::new("products"))
            .expr_as(invoice_total_expr(), Alias::new("total"))
            .expr_as(invoice_paid_amount_expr(), Alias::new("paid_amount"))
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Invoices, invoices::Column::ClientId)),
            )
            .cond_where(
                Cond::all()

                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(invoice_search_condition(&args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Invoices, invoices::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_from.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', invoices.created_at) >= strftime('%Y-%m-%d', ?)",
                        args.created_from,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.created_to.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', invoices.created_at) <= strftime('%Y-%m-%d', ?)",
                        args.created_to,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);

        match args.sort.as_deref() {
            Some("identifier") => {
                query.order_by(
                    (Invoices, invoices::Column::Identifier),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("full_name") => {
                query.order_by(
                    (Clients, clients::Column::FullName),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("products") => {
                query.order_by_expr(
                    Expr::cust("products"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("status") => {
                query.order_by(
                    (Invoices, invoices::Column::Status),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("total") => {
                query.order_by_expr(
                    Expr::cust("total"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("paid_amount") => {
                query.order_by_expr(
                    Expr::cust("paid_amount"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("created_at") => {
                query.order_by(
                    (Invoices, invoices::Column::CreatedAt),
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by((Invoices, invoices::Column::CreatedAt), Order::Desc);
            }
        }

        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);
        let result = SelectInvoices::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(InvoicesResponse {
            count,
            invoices: result,
        })
    }

    pub async fn get_invoice(db: &DbConn, id: String) -> Result<InvoiceWithClient, DbErr> {
        let invoice = Invoices::find_by_id(id.clone())
            .find_also_related(Clients)
            .one(db)
            .await?;

        match invoice {
            Some(invoice) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((InvoiceItems, invoice_items::Column::Id)),
                        Expr::col((InvoiceItems, invoice_items::Column::InventoryId)),
                        Expr::col((InvoiceItems, invoice_items::Column::Price)),
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(InvoiceItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InvoiceItems, invoice_items::Column::ProductId)),
                    )
                    .cond_where(
                        Expr::col((InvoiceItems, invoice_items::Column::InvoiceId)).eq(id.clone()),
                    )
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectInvoicesItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let total = invoice_total_by_id(db, &id).await?;
                let paid_amount = invoice_paid_by_id(db, &id).await?;
                let payments = invoice_payments_by_id(db, &id).await?;

                let (invoice_data, client_data) = invoice;
                let client = client_data.unwrap();

                Ok(InvoiceWithClient {
                    id: invoice_data.id,
                    client_id: invoice_data.client_id,
                    paid_amount,
                    total,
                    created_at: invoice_data.created_at.to_string(),
                    status: invoice_data.status,
                    identifier: invoice_data.identifier,
                    full_name: client.full_name,
                    email: client.email,
                    address: client.address,
                    phone_number: client.phone_number,
                    items,
                    payments,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no invoice"))),
        }
    }

    pub async fn list_invoice_products(
        db: &DbConn,
        id: String,
    ) -> Result<Vec<InvoiceProductItem>, DbErr> {
        let invoice_products = InvoiceItems::find()
            .select_only()
            .columns([
                invoice_items::Column::Price,
                invoice_items::Column::Quantity,
            ])
            .exprs([Expr::col((Products, products::Column::Name))])
            .join(JoinType::Join, invoice_items::Relation::Products.def())
            .filter(Expr::col((InvoiceItems, invoice_items::Column::InvoiceId)).eq(id))
            .into_model::<InvoiceProductItem>()
            .all(db)
            .await?;

        Ok(invoice_products)
    }

    pub async fn get_invoice_details(
        db: &DbConn,
        id: String,
    ) -> Result<InvoiceDetailsResponse, DbErr> {
        let (sql, values) = Query::select()
            .from(Invoices)
            .exprs([
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Invoices, invoices::Column::Id)),
                Expr::col((Invoices, invoices::Column::Status)),
                Expr::col((Invoices, invoices::Column::Identifier)),
                Expr::col((Invoices, invoices::Column::CreatedAt)),
                Expr::col((Invoices, invoices::Column::OrderId)),
            ])
            .expr_as(invoice_total_expr(), Alias::new("total"))
            .expr_as(invoice_paid_amount_expr(), Alias::new("paid_amount"))
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Invoices, invoices::Column::ClientId)),
            )
            .cond_where(Expr::col((Invoices, invoices::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let invoice = SelectInvoiceDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match invoice {
            Some(invoice) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((Products, products::Column::Name)),
                        Expr::col((InvoiceItems, invoice_items::Column::Price)),
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity)),
                    ])
                    .from(InvoiceItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InvoiceItems, invoice_items::Column::ProductId)),
                    )
                    .cond_where(
                        Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                            .eq(invoice.id.clone()),
                    )
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectInvoicesItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                let payments = invoice_payments_by_id(db, &invoice.id).await?;

                Ok(InvoiceDetailsResponse {
                    id: invoice.id,
                    paid_amount: invoice.paid_amount,
                    created_at: invoice.created_at,
                    status: invoice.status,
                    identifier: invoice.identifier,
                    total: invoice.total,
                    client: InvoiceClientInfo {
                        full_name: invoice.full_name,
                        email: invoice.email,
                        address: invoice.address,
                        phone_number: invoice.phone_number,
                    },
                    items,
                    payments,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no invoice"))),
        }
    }

    pub async fn create_invoice(
        db: &DbConn,
        invoice: NewInvoice,
    ) -> Result<String, TransactionError<DbErr>> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                let order_id = match invoice.order_id.clone() {
                    Some(order_id) => order_id,
                    None => {
                        let created_order = OrderActiveModel {
                            client_id: ActiveValue::Set(invoice.client_id.clone()),
                            status: ActiveValue::Set("PENDING".to_string()),
                            ..Default::default()
                        }
                        .insert(txn)
                        .await?;

                        created_order.id
                    }
                };

                let created_invoice = InvoiceActiveModel {
                    client_id: ActiveValue::Set(invoice.client_id),
                    status: ActiveValue::Set(invoice.status),
                    order_id: ActiveValue::Set(order_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let mut invoice_items = Vec::<InvoiceItemActiveModel>::new();
                for item in invoice.items {
                    let created_inventory = InventoryActiveModel {
                        product_id: ActiveValue::Set(item.product_id.clone()),
                        quantity: ActiveValue::Set(item.quantity as f32),
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

    pub async fn update_invoice(
        db: &DbConn,
        invoice: UpdateInvoice,
    ) -> Result<(), TransactionError<DbErr>> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let invoice_model = Invoices::find_by_id(invoice.id.clone()).one(txn).await?;
                let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
                invoice_active.client_id = ActiveValue::Set(invoice.client_id);
                invoice_active.status = ActiveValue::Set(invoice.status);
                invoice_active.save(txn).await?;

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
                            if let Some(inv_id) = item.inventory_id {
                                item_active.inventory_id = ActiveValue::Set(Some(inv_id));
                            }
                            item_active.save(txn).await?;
                        }
                        None => {
                            let created_inventory = InventoryActiveModel {
                                product_id: ActiveValue::Set(item.product_id.clone()),
                                quantity: ActiveValue::Set(item.quantity as f32),
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

    pub async fn add_invoice_payment(
        db: &DbConn,
        payment: AddInvoicePayment,
    ) -> Result<String, TransactionError<DbErr>> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                if payment.amount <= 0.0 {
                    return Err(DbErr::Custom(
                        "payment amount must be greater than zero".into(),
                    ));
                }

                let invoice_model = Invoices::find_by_id(payment.invoice_id.clone())
                    .one(txn)
                    .await?;
                let invoice_model =
                    invoice_model.ok_or_else(|| DbErr::RecordNotFound("no invoice".to_string()))?;

                if invoice_model.is_deleted {
                    return Err(DbErr::Custom(
                        "cannot add payment to a deleted invoice".into(),
                    ));
                }

                let total = invoice_total_by_id(txn, &payment.invoice_id).await?;
                let paid_amount = invoice_paid_by_id(txn, &payment.invoice_id).await?;
                let remaining = (total - paid_amount).max(0.0);

                if payment.amount > remaining {
                    return Err(DbErr::Custom(format!(
                        "payment amount exceeds unpaid amount ({remaining})"
                    )));
                }

                let payment_date = parse_payment_date(&payment.payment_date)?;
                let created_payment = InvoicePaymentActiveModel {
                    invoice_id: ActiveValue::Set(payment.invoice_id.clone()),
                    payment_date: ActiveValue::Set(payment_date),
                    description: ActiveValue::Set(payment.description),
                    amount: ActiveValue::Set(payment.amount),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                if let Some(next_status) = normalize_invoice_status(
                    &invoice_model.status,
                    total,
                    paid_amount + payment.amount,
                ) {
                    let mut invoice_active: InvoiceActiveModel = invoice_model.into();
                    invoice_active.status = ActiveValue::Set(next_status);
                    invoice_active.save(txn).await?;
                }

                Ok(created_payment.id)
            })
        })
        .await
    }

    pub async fn create_invoice_from_delivery_note(
        db: &DbConn,
        id: String,
    ) -> Result<String, TransactionError<DbErr>> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                match Invoices::find()
                    .filter(invoices::Column::DeliveryNoteId.eq(&id))
                    .one(txn)
                    .await?
                {
                    Some(invoice) => Ok(invoice.id),
                    None => match DeliveryNotes::find_by_id(&id).one(txn).await? {
                        Some(delivery_note) => {
                            let invoice = InvoiceActiveModel {
                                client_id: ActiveValue::Set(delivery_note.client_id),
                                status: ActiveValue::Set("DRAFT".to_string()),
                                order_id: ActiveValue::Set(delivery_note.order_id),
                                delivery_note_id: ActiveValue::Set(Some(delivery_note.id.clone())),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            let delivery_note_items = DeliveryNoteItems::find()
                                .filter(delivery_note_items::Column::DeliveryNoteId.eq(&id))
                                .all(txn)
                                .await?;

                            let mut invoice_items = Vec::<InvoiceItemActiveModel>::new();
                            for item in delivery_note_items {
                                invoice_items.push(InvoiceItemActiveModel {
                                    invoice_id: ActiveValue::Set(invoice.id.clone()),
                                    product_id: ActiveValue::Set(item.product_id),
                                    price: ActiveValue::Set(item.price as f64),
                                    quantity: ActiveValue::Set(item.quantity as f64),
                                    inventory_id: ActiveValue::Set(None),
                                    ..Default::default()
                                });
                            }

                            if !invoice_items.is_empty() {
                                InvoiceItems::insert_many(invoice_items).exec(txn).await?;
                            }

                            Ok(invoice.id)
                        }
                        None => Err(DbErr::RecordNotFound("delivery note not found".to_string())),
                    },
                }
            })
        })
        .await
    }

    pub async fn update_invoice_status(
        db: &DbConn,
        data: UpdateInvoiceStatus,
    ) -> Result<(), DbErr> {
        let next_status = InvoiceStatus::from_str(&data.status)
            .ok_or_else(|| DbErr::Custom(format!("invalid invoice status: {}", data.status)))?;

        let invoice_model = Invoices::find_by_id(data.id).one(db).await?;
        let invoice =
            invoice_model.ok_or_else(|| DbErr::RecordNotFound("invoice not found".to_string()))?;

        let current_status = InvoiceStatus::from_str(&invoice.status).ok_or_else(|| {
            DbErr::Custom(format!("corrupted invoice status: {}", invoice.status))
        })?;

        if !current_status.is_valid_transition(&next_status) {
            return Err(DbErr::Custom(format!(
                "invalid status transition from {} to {}",
                current_status.as_str(),
                next_status.as_str()
            )));
        }

        let mut invoice_active: InvoiceActiveModel = invoice.into();
        invoice_active.status = ActiveValue::Set(next_status.as_str().to_string());
        match invoice_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_invoice(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let invoice_model = Invoices::find_by_id(id).one(db).await?;
        let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
        invoice_active.is_deleted = ActiveValue::Set(true);
        match invoice_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_invoice_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let item_model = InvoiceItems::find_by_id(&id).one(db).await?;
        match item_model {
            Some(item_model) => {
                let res = item_model.delete(db).await?;
                Ok(res.rows_affected)
            }
            None => Ok(0),
        }
    }
}
