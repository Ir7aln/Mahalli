use super::dto::*;
use crate::{OrderStatus, QuoteStatus};
use sea_orm::{
    sea_query::{Alias, Cond, Expr, Func, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients,
    inventory_transactions::{self, ActiveModel as InventoryActiveModel},
    order_items::{self, ActiveModel as OrderItemActiveModel},
    orders::{self, ActiveModel as OrderActiveModel},
    prelude::*,
    products, quote_items,
    quotes::ActiveModel as QuoteActiveModel,
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn order_search_condition(search: &str) -> Cond {
    let pattern = format!("%{}%", search);
    Cond::any()
        .add(Expr::col((Clients, clients::Column::FullName)).like(pattern.clone()))
        .add(Expr::col((Orders, orders::Column::Identifier)).like(pattern))
}

pub struct OrdersService;

impl OrdersService {
    pub async fn list_orders(db: &DbConn, args: ListOrdersArgs) -> Result<OrdersResponse, DbErr> {
        let count = Orders::find()
            .join(JoinType::Join, orders::Relation::Clients.def())
            .filter(
                Cond::all()
                    .add(Expr::col((Orders, orders::Column::IsDeleted)).eq(false))
                    .add(order_search_condition(&args.search)),
            )
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Orders, orders::Column::Status)).eq(v))
            })
            .apply_if(args.created_from.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', orders.created_at) >= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.created_to.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', orders.created_at) <= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Orders)
            .exprs([
                Expr::col((Orders, orders::Column::Id)),
                Expr::col((Orders, orders::Column::Status)),
                Expr::col((Orders, orders::Column::Identifier)),
                Expr::col((Orders, orders::Column::CreatedAt)),
                Expr::col((Orders, orders::Column::ClientId)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::Ice)),
                Expr::col((Clients, clients::Column::IfNumber)),
                Expr::col((Clients, clients::Column::Rc)),
                Expr::col((Clients, clients::Column::Patente)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::count(Expr::col(
                        inventory_transactions::Column::Quantity,
                    ))),
                    Expr::val(0i64),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        ))
                        .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .left_join(
                InventoryTransactions,
                Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Orders, orders::Column::ClientId)),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Orders, orders::Column::IsDeleted)).eq(false))
                    .add(order_search_condition(&args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Orders, orders::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_from.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', orders.created_at) >= strftime('%Y-%m-%d', ?)",
                        args.created_from,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.created_to.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', orders.created_at) <= strftime('%Y-%m-%d', ?)",
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
                    (Orders, orders::Column::Identifier),
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
                    (Orders, orders::Column::Status),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("total") => {
                query.order_by_expr(
                    Expr::cust("total"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("created_at") => {
                query.order_by(
                    (Orders, orders::Column::CreatedAt),
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by((Orders, orders::Column::CreatedAt), Order::Desc);
            }
        }
        query.group_by_col((Orders, orders::Column::Id));

        if args.total_min.is_some() {
            query.and_having(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price))
                        .mul(Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))),
                )
                .gte(args.total_min.unwrap_or(0.0)),
            );
        }
        if args.total_max.is_some() {
            query.and_having(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price))
                        .mul(Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))),
                )
                .lte(args.total_max.unwrap_or(f64::MAX)),
            );
        }

        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

        let result = SelectOrders::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(OrdersResponse {
            count,
            orders: result,
        })
    }

    pub async fn get_order(db: &DbConn, id: String) -> Result<OrderWithClient, DbErr> {
        let order = Orders::find_by_id(id.clone())
            .find_also_related(Clients)
            .one(db)
            .await?;

        match order {
            Some(order) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((OrderItems, order_items::Column::Id)),
                        Expr::col((OrderItems, order_items::Column::InventoryId)),
                        Expr::col((OrderItems, order_items::Column::Price)),
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        )),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(OrderItems)
                    .join(
                        JoinType::Join,
                        InventoryTransactions,
                        Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                            .equals((OrderItems, order_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id)).equals((
                            InventoryTransactions,
                            inventory_transactions::Column::ProductId,
                        )),
                    )
                    .cond_where(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectOrdersItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let client = order.1.unwrap();
                Ok(OrderWithClient {
                    id: order.0.id,
                    client_id: order.0.client_id,
                    created_at: order.0.created_at.to_string(),
                    status: order.0.status,
                    identifier: order.0.identifier,
                    full_name: client.full_name,
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no order"))),
        }
    }

    pub async fn list_order_products(
        db: &DbConn,
        id: String,
    ) -> Result<Vec<OrderProductItem>, DbErr> {
        let order_products = OrderItems::find()
            .select_only()
            .columns([order_items::Column::Price])
            .exprs([
                Expr::col((Products, products::Column::Name)),
                Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                )),
            ])
            .join(
                JoinType::Join,
                order_items::Relation::InventoryTransactions.def(),
            )
            .join(
                JoinType::Join,
                inventory_transactions::Relation::Products.def(),
            )
            .filter(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
            .into_model::<OrderProductItem>()
            .all(db)
            .await?;

        Ok(order_products)
    }

    pub async fn get_order_details(db: &DbConn, id: String) -> Result<OrderDetailsResponse, DbErr> {
        let (sql, values) = Query::select()
            .from(Orders)
            .exprs([
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Orders, orders::Column::Id)),
                Expr::col((Orders, orders::Column::Status)),
                Expr::col((Orders, orders::Column::Identifier)),
                Expr::col((Orders, orders::Column::CreatedAt)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        ))
                        .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .left_join(
                InventoryTransactions,
                Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Orders, orders::Column::ClientId)),
            )
            .cond_where(Expr::col((Orders, orders::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let order = SelectOrderDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match order {
            Some(order) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((OrderItems, order_items::Column::Price)),
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        )),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .from(OrderItems)
                    .join(
                        JoinType::Join,
                        InventoryTransactions,
                        Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                            .equals((OrderItems, order_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id)).equals((
                            InventoryTransactions,
                            inventory_transactions::Column::ProductId,
                        )),
                    )
                    .cond_where(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectOrdersItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                Ok(OrderDetailsResponse {
                    id: order.id,
                    created_at: order.created_at,
                    status: order.status,
                    total: order.total,
                    identifier: order.identifier,
                    client: OrderClientInfo {
                        full_name: order.full_name,
                        email: order.email,
                        address: order.address,
                        phone_number: order.phone_number,
                    },
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no order"))),
        }
    }

    pub async fn create_order(
        db: &DbConn,
        order: NewOrder,
    ) -> Result<String, TransactionError<DbErr>> {
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
                        source_type: ActiveValue::Set("ORDER".to_string()),
                        source_id: ActiveValue::Set(Some(created_order.id.clone())),
                        unit_price: ActiveValue::Set(Some(item.price as f32)),
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
                if !items.is_empty() {
                    OrderItems::insert_many(items).exec(txn).await?;
                }

                Ok(created_order.id)
            })
        })
        .await
    }

    pub async fn update_order(
        db: &DbConn,
        order: UpdateOrder,
    ) -> Result<(), TransactionError<DbErr>> {
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
                                source_type: ActiveValue::Set("ORDER".to_string()),
                                source_id: ActiveValue::Set(Some(order.id.clone())),
                                unit_price: ActiveValue::Set(Some(item.price as f32)),
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
                if !items.is_empty() {
                    OrderItems::insert_many(items).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
    }

    pub async fn create_order_from_quote(
        db: &DbConn,
        id: String,
    ) -> Result<String, TransactionError<DbErr>> {
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
                                client_id: ActiveValue::Set(quote.client_id.clone()),
                                status: ActiveValue::Set("PENDING".to_string()),
                                quote_id: ActiveValue::Set(Some(quote.id.clone())),
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
                                    source_type: ActiveValue::Set("ORDER".to_string()),
                                    source_id: ActiveValue::Set(Some(order.id.clone())),
                                    unit_price: ActiveValue::Set(Some(item.price as f32)),
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
                            if !items.is_empty() {
                                OrderItems::insert_many(items).exec(txn).await?;
                            }

                            let mut quote_active: QuoteActiveModel = quote.into();
                            quote_active.status =
                                ActiveValue::Set(QuoteStatus::Accepted.as_str().to_string());
                            quote_active.update(txn).await?;

                            Ok(order.id)
                        }
                        None => Err(DbErr::RecordNotFound("no quote".to_string())),
                    },
                }
            })
        })
        .await
    }

    pub async fn update_order_status(db: &DbConn, data: UpdateOrderStatus) -> Result<(), DbErr> {
        let next_status = OrderStatus::from_str(&data.status)
            .ok_or_else(|| DbErr::Custom(format!("invalid order status: {}", data.status)))?;

        let order_model = Orders::find_by_id(data.id).one(db).await?;
        let order =
            order_model.ok_or_else(|| DbErr::RecordNotFound("order not found".to_string()))?;

        let current_status = OrderStatus::from_str(&order.status)
            .ok_or_else(|| DbErr::Custom(format!("corrupted order status: {}", order.status)))?;

        if !current_status.is_valid_transition(&next_status) {
            return Err(DbErr::Custom(format!(
                "invalid status transition from {} to {}",
                current_status.as_str(),
                next_status.as_str()
            )));
        }

        let mut order_active: OrderActiveModel = order.into();
        order_active.status = ActiveValue::Set(next_status.as_str().to_string());
        match order_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_order(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let order_model = Orders::find_by_id(id).one(db).await?;
        let mut order_active: OrderActiveModel = order_model.unwrap().into();
        order_active.is_deleted = ActiveValue::Set(true);
        match order_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_order_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let item_model = OrderItems::find_by_id(id).one(db).await?;
        match item_model {
            Some(item_model) => {
                let res = item_model.delete(db).await?;
                Ok(res.rows_affected)
            }
            None => Ok(0),
        }
    }
}
