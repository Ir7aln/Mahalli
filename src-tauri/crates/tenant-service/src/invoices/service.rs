use super::dto::*;
use sea_orm::{
    sea_query::{Alias, Cond, Expr, Func, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients::{self, Entity as Clients},
    inventory_transactions::{
        ActiveModel as InventoryActiveModel, Entity as InventoryTransactions,
    },
    invoice_items::{self, ActiveModel as InvoiceItemActiveModel, Entity as InvoiceItems},
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
                    .add(Expr::col((Invoices, invoices::Column::IsArchived)).eq(false))
                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(
                        Cond::any()
                            .add(
                                Expr::col((Clients, clients::Column::FullName))
                                    .like(format!("{}%", args.search)),
                            )
                            .add(
                                Expr::col((Invoices, invoices::Column::Identifier))
                                    .like(format!("{}%", args.search)),
                            ),
                    ),
            )
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Invoices, invoices::Column::Status)).eq(v))
            })
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', invoices.created_at) = strftime('%Y-%m-%d', ?)",
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
                Expr::col((Invoices, invoices::Column::PaidAmount)),
                Expr::col((Clients, clients::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::count(Expr::col((
                        InvoiceItems,
                        invoice_items::Column::Id,
                    )))),
                    Expr::val(0i64),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Invoices, invoices::Column::ClientId)),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Invoices, invoices::Column::IsArchived)).eq(false))
                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(
                        Cond::any()
                            .add(
                                Expr::col((Clients, clients::Column::FullName))
                                    .like(format!("{}%", args.search)),
                            )
                            .add(
                                Expr::col((Invoices, invoices::Column::Identifier))
                                    .like(format!("{}%", args.search)),
                            ),
                    ),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Invoices, invoices::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', invoices.created_at) = strftime('%Y-%m-%d', ?)",
                        args.created_at,
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
                query.order_by(
                    (Invoices, invoices::Column::PaidAmount),
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
        query.group_by_col((Invoices, invoices::Column::Id));
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

                let (invoice_data, client_data) = invoice;
                let client = client_data.unwrap();

                Ok(InvoiceWithClient {
                    id: invoice_data.id,
                    client_id: invoice_data.client_id,
                    paid_amount: invoice_data.paid_amount,
                    created_at: invoice_data.created_at.to_string(),
                    status: invoice_data.status,
                    identifier: invoice_data.identifier,
                    full_name: client.full_name,
                    email: client.email,
                    address: client.address,
                    phone_number: client.phone_number,
                    items,
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
                Expr::col((Invoices, invoices::Column::PaidAmount)),
                Expr::col((Invoices, invoices::Column::CreatedAt)),
                Expr::col((Invoices, invoices::Column::OrderId)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
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
                invoice_active.paid_amount = ActiveValue::Set(invoice.paid_amount);
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

    pub async fn create_invoice_from_order(
        db: &DbConn,
        id: String,
    ) -> Result<String, TransactionError<DbErr>> {
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
                            let invoice = InvoiceActiveModel {
                                client_id: ActiveValue::Set(order.client_id),
                                paid_amount: ActiveValue::Set(0.0f32),
                                status: ActiveValue::Set("DRAFT".to_string()),
                                order_id: ActiveValue::Set(order.id),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

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
                                    price: ActiveValue::Set(item.price as f64),
                                    quantity: ActiveValue::Set(inv_txn.quantity as f64),
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

    pub async fn update_invoice_status(
        db: &DbConn,
        data: UpdateInvoiceStatus,
    ) -> Result<(), DbErr> {
        let invoice_model = Invoices::find_by_id(data.id).one(db).await?;
        let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
        invoice_active.status = ActiveValue::Set(data.status);
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
