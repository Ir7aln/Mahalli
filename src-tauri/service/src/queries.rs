use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement,
    },
    DatabaseConnection as DbConn, *,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{entities::*, models::*};

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct ListArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
}

pub struct QueriesService;

impl QueriesService {
    pub async fn list_products(db: &DbConn, args: ListArgs) -> Result<ProductsResponse, DbErr> {
        let count = Products::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Products, products::Column::IsArchived)).eq(false))
                    .add(Expr::col((Products, products::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Products, products::Column::Name))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Products)
            .exprs([
                Expr::col((Products, products::Column::Id)),
                Expr::col((Products, products::Column::Name)),
                Expr::col((Products, products::Column::Description)),
                Expr::col((Products, products::Column::Image)),
                Expr::col((Products, products::Column::CreatedAt)),
                Expr::col((Products, products::Column::SellingPrice)),
                Expr::col((Products, products::Column::PurchasePrice)),
                Expr::col((Products, products::Column::MinQuantity)),
            ])
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(InventoryTransactions)
                            .expr(Func::coalesce([
                                Func::sum(Expr::col(inventory_transactions::Column::Quantity))
                                    .into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .cond_where(
                                Cond::all()
                                    .add(
                                        inventory_transactions::Column::TransactionType
                                            .eq(String::from("IN")),
                                    )
                                    .add(
                                        Expr::col((
                                            InventoryTransactions,
                                            inventory_transactions::Column::ProductId,
                                        ))
                                        .equals((Products, products::Column::Id)),
                                    ),
                            )
                            .to_owned(),
                    )),
                )
                .sub(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(InventoryTransactions)
                            .expr(Func::coalesce([
                                Func::sum(Expr::col(inventory_transactions::Column::Quantity))
                                    .into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .join(
                                JoinType::Join,
                                OrderItems,
                                Expr::col((OrderItems, order_items::Column::InventoryId)).equals((
                                    InventoryTransactions,
                                    inventory_transactions::Column::Id,
                                )),
                            )
                            .join(
                                JoinType::Join,
                                Orders,
                                Expr::col((Orders, orders::Column::Id))
                                    .equals((OrderItems, order_items::Column::OrderId)),
                            )
                            .cond_where(
                                Cond::all()
                                    .add(
                                        Expr::col((
                                            InventoryTransactions,
                                            inventory_transactions::Column::ProductId,
                                        ))
                                        .equals((Products, products::Column::Id)),
                                    )
                                    .add(orders::Column::Status.is_in(["DELIVERED", "SHIPPED"]))
                                    .add(orders::Column::IsDeleted.eq(false)),
                            )
                            .to_owned(),
                    )),
                )),
                Alias::new("inventory"),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Products, products::Column::IsArchived)).eq(false))
                    .add(Expr::col((Products, products::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Products, products::Column::Name))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(products::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let products = SelectProducts::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(ProductsResponse { count, products })
    }
    pub async fn search_products(db: &DbConn, search: String) -> Result<Vec<ProductSearch>, DbErr> {
        let products = Products::find()
            .select_only()
            .expr_as_(Expr::col(products::Column::Name), "label")
            .expr_as_(Expr::col(products::Column::Id), "value")
            .expr_as_(Expr::col(products::Column::SellingPrice), "price")
            .filter(products::Column::IsDeleted.eq(false))
            .filter(products::Column::Name.like(format!("{}%", search)))
            .into_model::<ProductSearch>()
            .all(db)
            .await?;
        Ok(products)
    }
    //
    pub async fn list_clients(db: &DbConn, args: ListArgs) -> Result<ClientsResponse, DbErr> {
        let count = Clients::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Clients, clients::Column::IsArchived)).eq(false))
                    .add(Expr::col((Clients, clients::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Clients)
            .exprs([
                Expr::col((Clients, clients::Column::Id)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Image)),
                Expr::col((Clients, clients::Column::Email)),
            ])
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Invoices)
                            .expr(Expr::expr(Func::coalesce([
                                Func::sum(
                                    Expr::col((
                                        InventoryTransactions,
                                        inventory_transactions::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .into(),
                                Expr::val(0.0f64).into(),
                            ])))
                            .left_join(
                                Orders,
                                Expr::col((Orders, orders::Column::Id))
                                    .equals((Invoices, invoices::Column::OrderId)),
                            )
                            .inner_join(
                                OrderItems,
                                Expr::col((OrderItems, order_items::Column::OrderId))
                                    .equals((Orders, orders::Column::Id)),
                            )
                            .inner_join(
                                InventoryTransactions,
                                Expr::col((
                                    InventoryTransactions,
                                    inventory_transactions::Column::Id,
                                ))
                                .equals((OrderItems, order_items::Column::InventoryId)),
                            )
                            .cond_where(
                                Cond::all()
                                    .add(
                                        Expr::col((Invoices, invoices::Column::Status))
                                            .is_not_in(["CANCELLED", "DRAFT", "PAID"]),
                                    )
                                    .add(
                                        Expr::col((Invoices, invoices::Column::IsDeleted))
                                            .eq(false),
                                    )
                                    .add(
                                        Expr::col((Invoices, invoices::Column::ClientId))
                                            .equals((Clients, clients::Column::Id)),
                                    ),
                            )
                            .to_owned(),
                    )),
                )
                .sub(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Invoices)
                            .expr(Expr::expr(Func::coalesce([
                                Func::sum(Expr::col((Invoices, invoices::Column::PaidAmount)))
                                    .into(),
                                Expr::val(0.0f64).into(),
                            ])))
                            .cond_where(
                                Cond::all()
                                    .add(
                                        Expr::col((Invoices, invoices::Column::Status))
                                            .eq("PARTIALLY_PAID"),
                                    )
                                    .add(
                                        Expr::col((Invoices, invoices::Column::IsDeleted))
                                            .eq(false),
                                    )
                                    .add(
                                        Expr::col((Invoices, invoices::Column::ClientId))
                                            .equals((Clients, clients::Column::Id)),
                                    ),
                            )
                            .to_owned(),
                    )),
                )),
                Alias::new("credit"),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Clients, clients::Column::IsArchived)).eq(false))
                    .add(Expr::col((Clients, clients::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(clients::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let result = SelectClients::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(ClientsResponse {
            count,
            clients: result,
        })
    }
    pub async fn search_clients(db: &DbConn, search: String) -> Result<Vec<ClientSearch>, DbErr> {
        let clients = Clients::find()
            .select_only()
            .expr_as_(Expr::col(clients::Column::FullName), "label")
            .expr_as_(Expr::col(clients::Column::Id), "value")
            .filter(clients::Column::IsDeleted.eq(false))
            .filter(clients::Column::FullName.like(format!("{}%", search)))
            .into_model::<ClientSearch>()
            .all(db)
            .await?;
        Ok(clients)
    }
    pub async fn get_client(db: &DbConn, id: String) -> Result<ClientDetails, DbErr> {
        let client = Clients::find_by_id(id).one(db).await?;

        match client {
            Some(client) => Ok(ClientDetails {
                id: client.id,
                full_name: client.full_name,
                email: client.email,
                phone_number: client.phone_number,
                address: client.address,
                image: client.image,
            }),
            None => Err(DbErr::RecordNotFound(String::from("no client"))),
        }
    }
    //
    pub async fn list_suppliers(db: &DbConn, args: ListArgs) -> Result<SuppliersResponse, DbErr> {
        let count = Suppliers::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Suppliers, suppliers::Column::IsArchived)).eq(false))
                    .add(Expr::col((Suppliers, suppliers::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Suppliers, suppliers::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Suppliers)
            .exprs([
                Expr::col((Suppliers, suppliers::Column::Id)),
                Expr::col((Suppliers, suppliers::Column::FullName)),
                Expr::col((Suppliers, suppliers::Column::Address)),
                Expr::col((Suppliers, suppliers::Column::PhoneNumber)),
                Expr::col((Suppliers, suppliers::Column::Image)),
                Expr::col((Suppliers, suppliers::Column::Email)),
            ])
            .cond_where(
                Cond::all()
                    .add(Expr::col((Suppliers, suppliers::Column::IsArchived)).eq(false))
                    .add(Expr::col((Suppliers, suppliers::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Suppliers, suppliers::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(suppliers::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let result = SelectSuppliers::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(SuppliersResponse {
            count,
            suppliers: result,
        })
    }
    pub async fn search_suppliers(
        db: &DbConn,
        search: String,
    ) -> Result<Vec<SupplierSearch>, DbErr> {
        let suppliers = Suppliers::find()
            .select_only()
            .expr_as_(Expr::col(suppliers::Column::FullName), "label")
            .expr_as_(Expr::col(suppliers::Column::Id), "value")
            .filter(suppliers::Column::IsDeleted.eq(false))
            .filter(suppliers::Column::FullName.like(format!("{}%", search)))
            .into_model::<SupplierSearch>()
            .all(db)
            .await?;
        Ok(suppliers)
    }
    //
    pub async fn list_orders(db: &DbConn, args: ListArgs) -> Result<OrdersResponse, DbErr> {
        let count = Orders::find()
            .join(JoinType::Join, orders::Relation::Clients.def())
            .filter(
                Cond::all()
                    .add(Expr::col((Orders, orders::Column::IsArchived)).eq(false))
                    .add(Expr::col((Orders, orders::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Orders, orders::Column::Status)).eq(v))
            })
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', orders.created_at) = strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Orders)
            .exprs([
                Expr::col((Orders, orders::Column::Id)),
                Expr::col((Orders, orders::Column::Status)),
                Expr::col((Orders, orders::Column::Identifier)),
                Expr::col((Orders, orders::Column::CreatedAt)),
                Expr::col((Orders, orders::Column::ClientId)),
                Expr::col((Clients, clients::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::count(Expr::col(inventory_transactions::Column::Quantity)).into(),
                    Expr::val(0i64).into(),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        ))
                        .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
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
                    .add(Expr::col((Orders, orders::Column::IsArchived)).eq(false))
                    .add(Expr::col((Orders, orders::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Orders, orders::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', orders.created_at) = strftime('%Y-%m-%d', ?)",
                        args.created_at,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by((Orders, orders::Column::CreatedAt), Order::Desc)
            .group_by_col((Orders, orders::Column::Id))
            .to_owned()
            .build(SqliteQueryBuilder);

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
                    created_at: order.0.created_at,
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
                    Func::sum(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        ))
                        .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
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
    //
    pub async fn list_invoices(db: &DbConn, args: ListArgs) -> Result<InvoicesResponse, DbErr> {
        let count = Invoices::find()
            .join(JoinType::Join, invoices::Relation::Clients.def())
            .filter(
                Cond::all()
                    .add(Expr::col((Invoices, invoices::Column::IsArchived)).eq(false))
                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
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

        let (sql, values) = Query::select()
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
                    Func::count(Expr::col((InvoiceItems, invoice_items::Column::Id))).into(),
                    Expr::val(0i64).into(),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
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
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
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
            .offset((args.page - 1) * args.limit)
            .order_by((Invoices, invoices::Column::CreatedAt), Order::Desc)
            .group_by_col((Invoices, invoices::Column::Id))
            .to_owned()
            .build(SqliteQueryBuilder);

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
                    created_at: invoice_data.created_at,
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
                    Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
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
    //
    pub async fn list_quotes(db: &DbConn, args: ListArgs) -> Result<QuotesResponse, DbErr> {
        let count = Quotes::find()
            .join(JoinType::Join, quotes::Relation::Clients.def())
            .filter(
                Cond::all()
                    .add(Expr::col((Quotes, quotes::Column::IsArchived)).eq(false))
                    .add(Expr::col((Quotes, quotes::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', quotes.created_at) = strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Quotes)
            .exprs([
                Expr::col((Quotes, quotes::Column::Id)),
                Expr::col((Quotes, quotes::Column::CreatedAt)),
                Expr::col((Quotes, quotes::Column::ClientId)),
                Expr::col((Quotes, quotes::Column::Identifier)),
                Expr::col((Clients, clients::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::count(Expr::col(inventory_transactions::Column::Quantity)).into(),
                    Expr::val(0i64).into(),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((QuoteItems, quote_items::Column::Quantity))
                            .mul(Expr::col((QuoteItems, quote_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                QuoteItems,
                Expr::col((QuoteItems, quote_items::Column::QuoteId))
                    .equals((Quotes, quotes::Column::Id)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Quotes, quotes::Column::ClientId)),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Quotes, quotes::Column::IsArchived)).eq(false))
                    .add(Expr::col((Quotes, quotes::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', quotes.created_at) = strftime('%Y-%m-%d', ?)",
                        args.created_at,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by((Quotes, quotes::Column::CreatedAt), Order::Desc)
            .group_by_col((Quotes, quotes::Column::Id))
            .to_owned()
            .build(SqliteQueryBuilder);

        let result = SelectQuotes::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(QuotesResponse {
            count,
            quotes: result,
        })
    }
    pub async fn get_quote(db: &DbConn, id: String) -> Result<QuoteWithClient, DbErr> {
        let quote = Quotes::find_by_id(id.clone())
            .find_also_related(Clients)
            .one(db)
            .await?;

        match quote {
            Some(quote) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((QuoteItems, quote_items::Column::Id)),
                        Expr::col((QuoteItems, quote_items::Column::Price)),
                        Expr::col((QuoteItems, quote_items::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(QuoteItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((QuoteItems, quote_items::Column::ProductId)),
                    )
                    .cond_where(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectQuotesItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let client = quote.1.unwrap();
                Ok(QuoteWithClient {
                    id: quote.0.id,
                    client_id: quote.0.client_id,
                    created_at: quote.0.created_at,
                    identifier: quote.0.identifier,
                    full_name: client.full_name,
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no quote"))),
        }
    }
    pub async fn list_quote_products(
        db: &DbConn,
        id: String,
    ) -> Result<Vec<QuoteProductItem>, DbErr> {
        let quote_products = QuoteItems::find()
            .select_only()
            .columns([quote_items::Column::Price])
            .exprs([
                Expr::col((Products, products::Column::Name)),
                Expr::col((QuoteItems, quote_items::Column::Quantity)),
            ])
            .join(JoinType::Join, quote_items::Relation::Products.def())
            .filter(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
            .into_model::<QuoteProductItem>()
            .all(db)
            .await?;

        Ok(quote_products)
    }
    pub async fn get_quote_details(db: &DbConn, id: String) -> Result<QuoteDetailsResponse, DbErr> {
        let (sql, values) = Query::select()
            .from(Quotes)
            .exprs([
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Quotes, quotes::Column::Id)),
                Expr::col((Quotes, quotes::Column::Identifier)),
                Expr::col((Quotes, quotes::Column::CreatedAt)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((QuoteItems, quote_items::Column::Quantity))
                            .mul(Expr::col((QuoteItems, quote_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                QuoteItems,
                Expr::col((QuoteItems, quote_items::Column::QuoteId))
                    .equals((Quotes, quotes::Column::Id)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Quotes, quotes::Column::ClientId)),
            )
            .cond_where(Expr::col((Quotes, quotes::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let quote = SelectQuoteDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match quote {
            Some(quote) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((QuoteItems, quote_items::Column::Price)),
                        Expr::col((QuoteItems, quote_items::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .from(QuoteItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((QuoteItems, quote_items::Column::ProductId)),
                    )
                    .cond_where(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectQuotesItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                Ok(QuoteDetailsResponse {
                    id: quote.id,
                    created_at: quote.created_at,
                    identifier: quote.identifier,
                    total: quote.total,
                    client: QuoteClientInfo {
                        full_name: quote.full_name,
                        email: quote.email,
                        address: quote.address,
                        phone_number: quote.phone_number,
                    },
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no quote"))),
        }
    }
    //
    pub async fn list_inventory(db: &DbConn, args: ListArgs) -> Result<InventoryResponse, DbErr> {
        let count = InventoryTransactions::find()
            .join(JoinType::Join, inventory_transactions::Relation::Products.def())
            .join(JoinType::LeftJoin, inventory_transactions::Relation::OrderItems.def())
            .join(JoinType::LeftJoin, order_items::Relation::Orders.def())
            .filter(
                Cond::all().add(
                    Expr::expr(Func::coalesce([
                        Expr::col((Orders, orders::Column::Status)).into(),
                        Expr::expr("PENDING").into(),
                    ])).eq("CANCELLED").not(),
                ).add(Expr::expr(Func::coalesce([
                    Expr::col((Orders, orders::Column::IsDeleted)).into(),
                    Expr::expr(false).into(),
                ])).eq(false)),
            ).apply_if(Some(args.search.clone()), |query, v| {
                query.filter(Expr::col((Products, products::Column::Name)).like(format!("{}%", v)))
            }).apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)).eq(v))
            }).apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values("strftime('%Y-%m-%d', inventory_transactions.created_at) = strftime('%Y-%m-%d', ?)", [v]))
            }).count(db).await?;

        let (sql, values) = Query::select().from(InventoryTransactions).exprs([
            Expr::col((InventoryTransactions, inventory_transactions::Column::Id)),
            Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)),
            Expr::col((Products, products::Column::Name)),
            Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)),
            Expr::col((InventoryTransactions, inventory_transactions::Column::CreatedAt)),
        ]).expr_as(
            Func::coalesce([
                Expr::col((OrderItems, order_items::Column::Price)).into(),
                Expr::col((Products, products::Column::PurchasePrice)).into(),
            ]),
            Alias::new("price"),
        ).join(
            JoinType::Join,
            Products,
            Expr::col((Products, products::Column::Id)).equals((InventoryTransactions, inventory_transactions::Column::ProductId)),
        ).join(
            JoinType::LeftJoin,
            OrderItems,
            Expr::col((OrderItems, order_items::Column::InventoryId)).equals((InventoryTransactions, inventory_transactions::Column::Id)),
        ).join(
            JoinType::LeftJoin,
            Orders,
            Expr::col((Orders, orders::Column::Id)).equals((OrderItems, order_items::Column::OrderId)),
        ).cond_where(
            Cond::all().add(
                Expr::expr(Func::coalesce([
                    Expr::col((Orders, orders::Column::Status)).into(),
                    Expr::expr("PENDING").into(),
                ])).eq("CANCELLED").not(),
            ).add(Expr::expr(Func::coalesce([
                Expr::col((Orders, orders::Column::IsDeleted)).into(),
                Expr::expr(false).into(),
            ])).eq(false)),
        ).and_where(Expr::col((Products, products::Column::Name)).like(format!("{}%", args.search))).conditions(
            args.status.clone().is_some(),
            |x| {
                x.and_where(Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)).eq(args.status));
            },
            |_| {},
        ).conditions(
            args.created_at.clone().is_some(),
            |x| {
                x.and_where(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', inventory_transactions.created_at) = strftime('%Y-%m-%d', ?)",
                    args.created_at,
                ));
            },
            |_| {},
        ).order_by_expr(
            Expr::col((InventoryTransactions, inventory_transactions::Column::CreatedAt)).into(),
            Order::Desc,
        ).limit(args.limit).offset((args.page - 1) * args.limit).to_owned().build(SqliteQueryBuilder);

        let result = SelectInventory::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(InventoryResponse {
            count,
            inventory: result,
        })
    }
    pub async fn list_inventory_stats(db: &DbConn) -> Result<Vec<SelectTransaction>, DbErr> {
        let (sql, values) = Query::select()
            .from(InventoryTransactions)
            .columns([(
                InventoryTransactions,
                inventory_transactions::Column::TransactionType,
            )])
            .expr_as(
                Expr::cust("strftime('%Y-%m', inventory_transactions.created_at)"),
                Alias::new("created_at"),
            )
            .expr_as(
                Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                ))
                .sum(),
                Alias::new("quantity"),
            )
            .expr_as(
                Func::sum(
                    Expr::expr(Func::coalesce([
                        Expr::col((OrderItems, order_items::Column::Price)).into(),
                        Expr::col((Products, products::Column::PurchasePrice)).into(),
                    ]))
                    .mul(Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::Quantity,
                    ))),
                ),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Products,
                Expr::col((Products, products::Column::Id)).equals((
                    InventoryTransactions,
                    inventory_transactions::Column::ProductId,
                )),
            )
            .join(
                JoinType::LeftJoin,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::InventoryId))
                    .equals((InventoryTransactions, inventory_transactions::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                Orders,
                Expr::col((Orders, orders::Column::Id))
                    .equals((OrderItems, order_items::Column::OrderId)),
            )
            .cond_where(
                Cond::all()
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Orders, orders::Column::Status)).into(),
                            Expr::expr("PENDING").into(),
                        ]))
                        .eq("CANCELLED")
                        .not(),
                    )
                    .add(Expr::cust(
                        "inventory_transactions.created_at >= DATETIME('now', '-3 month')",
                    )),
            )
            .add_group_by([
                Expr::cust("strftime('%Y-%m', inventory_transactions.created_at)"),
                Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::TransactionType,
                ))
                .into(),
            ])
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectTransaction::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(res)
    }
    //
    pub async fn list_top_products(db: &DbConn) -> Result<Vec<SelectTopProducts>, DbErr> {
        let (sql, values) = Query::select()
            .from(Products)
            .column((Products, products::Column::Name))
            .expr_as(
                Func::sum(Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                ))),
                Alias::new("quantity"),
            )
            .join(
                JoinType::Join,
                InventoryTransactions,
                Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::ProductId,
                ))
                .equals((Products, products::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::InventoryId))
                    .equals((InventoryTransactions, inventory_transactions::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                Orders,
                Expr::col((Orders, orders::Column::Id))
                    .equals((OrderItems, order_items::Column::OrderId)),
            )
            .cond_where(
                Cond::all()
                    .add(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::TransactionType,
                        ))
                        .eq("OUT"),
                    )
                    .add(
                        Expr::col((Orders, orders::Column::Status))
                            .eq("CANCELLED")
                            .not(),
                    ),
            )
            .add_group_by([Expr::col((Products, products::Column::Id)).into()])
            .order_by_expr(
                Func::sum(Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                )))
                .into(),
                Order::Desc,
            )
            .limit(10)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectTopProducts::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(res)
    }
    pub async fn list_top_clients(db: &DbConn) -> Result<Vec<SelectTops>, DbErr> {
        let (sql, values) = Query::select()
            .from(Clients)
            .column((Clients, clients::Column::FullName))
            .expr_as(
                Func::sum(Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                ))),
                Alias::new("quantity"),
            )
            .expr_as(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::Quantity,
                    ))),
                ),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Invoices,
                Expr::col((Invoices, invoices::Column::ClientId))
                    .equals((Clients, clients::Column::Id)),
            )
            .join(
                JoinType::Join,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Invoices, invoices::Column::OrderId)),
            )
            .join(
                JoinType::Join,
                InventoryTransactions,
                Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .cond_where(
                Cond::all().add(
                    Expr::expr(Expr::col((Invoices, invoices::Column::Status)))
                        .is_not_in(["CANCELLED", "DRAFT"]),
                ),
            )
            .add_group_by([Expr::col((Clients, clients::Column::Id)).into()])
            .order_by_expr(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::Quantity,
                    ))),
                )
                .into(),
                Order::Desc,
            )
            .limit(5)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectTops::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(res)
    }
    pub async fn list_top_suppliers(db: &DbConn) -> Result<Vec<SelectTops>, DbErr> {
        let (sql, values) = Query::select()
            .from(Suppliers)
            .column((Suppliers, suppliers::Column::FullName))
            .expr_as(
                Func::sum(Expr::col((
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                ))),
                Alias::new("quantity"),
            )
            .expr_as(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::Quantity,
                    ))),
                ),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Orders,
                Expr::col((Orders, orders::Column::ClientId))
                    .equals((Suppliers, suppliers::Column::Id)),
            )
            .join(
                JoinType::Join,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .join(
                JoinType::Join,
                InventoryTransactions,
                Expr::col((InventoryTransactions, inventory_transactions::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .cond_where(
                Cond::all().add(
                    Expr::expr(Expr::col((Orders, orders::Column::Status)))
                        .is_not_in(["CANCELLED", "PENDING"]),
                ),
            )
            .add_group_by([Expr::col((Suppliers, suppliers::Column::Id)).into()])
            .order_by_expr(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::Quantity,
                    ))),
                )
                .into(),
                Order::Desc,
            )
            .limit(5)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectTops::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(res)
    }
    pub async fn list_status_count(db: &DbConn) -> Result<StatusCountResponse, DbErr> {
        let (order_sql, order_values) = Query::select()
            .from(Orders)
            .column(orders::Column::Status)
            .expr_as(
                Func::count(Expr::col((Orders, orders::Column::Id))),
                Alias::new("status_count"),
            )
            .cond_where(
                Cond::all()
                    .add(orders::Column::IsDeleted.eq(false))
                    .add(orders::Column::IsArchived.eq(false)),
            )
            .group_by_col(orders::Column::Status)
            .to_owned()
            .build(SqliteQueryBuilder);

        let (invoice_sql, invoice_values) = Query::select()
            .from(Invoices)
            .column(invoices::Column::Status)
            .expr_as(
                Func::count(Expr::col((Invoices, invoices::Column::Id))),
                Alias::new("status_count"),
            )
            .cond_where(
                Cond::all()
                    .add(invoices::Column::IsDeleted.eq(false))
                    .add(invoices::Column::IsArchived.eq(false)),
            )
            .group_by_col(invoices::Column::Status)
            .to_owned()
            .build(SqliteQueryBuilder);

        let order_res = SelectStatusCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            order_sql,
            order_values,
        ))
        .all(db)
        .await?;

        let invoice_res = SelectStatusCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            invoice_sql,
            invoice_values,
        ))
        .all(db)
        .await?;

        Ok(StatusCountResponse {
            orders: order_res,
            invoices: invoice_res,
        })
    }
    pub async fn list_financial_metrics(db: &DbConn) -> Result<FinancialMetricsResponse, DbErr> {
        let (sql, values) = Query::select().expr_as(Expr::expr(
            Expr::expr(SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select().expr(Func::coalesce([
                        Func::sum(
                            Expr::case(
                                Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)).mul(Expr::col((OrderItems, order_items::Column::Price))),
                            ).case(
                                Expr::col((Invoices, invoices::Column::Status)).eq("PARTIALLY_PAID"),
                                Expr::col((Invoices, invoices::Column::PaidAmount)),
                            ).finally(Expr::val(0))
                        ).into(),
                        Expr::val(0.0).into(),
                    ])).from(InventoryTransactions).join(JoinType::InnerJoin, OrderItems,
                        Expr::col((OrderItems, order_items::Column::InventoryId)).equals((InventoryTransactions, inventory_transactions::Column::Id))).join(JoinType::InnerJoin, Orders,
                        Expr::col((Orders, orders::Column::Id)).equals((OrderItems, order_items::Column::OrderId))).join(JoinType::InnerJoin, Invoices,
                        Expr::col((Invoices, invoices::Column::OrderId)).equals((Orders, orders::Column::Id))).cond_where(
                        Expr::cust("invoices.created_at >= strftime('%Y-%m-01', CURRENT_DATE)").and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec!["PAID", "PARTIALLY_PAID"])).and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    ).to_owned()
                )),
            ))
        ), Alias::new("current_revenue")).expr_as(Expr::expr(
            Expr::expr(SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select().expr(Func::coalesce([
                        Func::sum(
                            Expr::case(
                                Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)).mul(Expr::col((OrderItems, order_items::Column::Price))),
                            ).case(
                                Expr::col((Invoices, invoices::Column::Status)).eq("PARTIALLY_PAID"),
                                Expr::col((Invoices, invoices::Column::PaidAmount)),
                            ).finally(Expr::val(0))
                        ).into(),
                        Expr::val(0.0).into(),
                    ])).from(InventoryTransactions).join(JoinType::InnerJoin, OrderItems,
                        Expr::col((OrderItems, order_items::Column::InventoryId)).equals((InventoryTransactions, inventory_transactions::Column::Id))).join(JoinType::InnerJoin, Orders,
                        Expr::col((Orders, orders::Column::Id)).equals((OrderItems, order_items::Column::OrderId))).join(JoinType::InnerJoin, Invoices,
                        Expr::col((Invoices, invoices::Column::OrderId)).equals((Orders, orders::Column::Id))).cond_where(
                        Expr::cust("invoices.created_at < strftime('%Y-%m-01', CURRENT_DATE)").and(Expr::cust("invoices.created_at >= strftime('%Y-%m-01', CURRENT_DATE, '-1 month')")).and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec!["PAID", "PARTIALLY_PAID"])).and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    ).to_owned()
                )),
            ))
        ), Alias::new("last_month_revenue")).expr_as(Expr::expr(
            Expr::expr(SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select().expr(Func::coalesce([
                        Func::sum(
                            Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)).mul(Expr::col((Products, products::Column::PurchasePrice)))
                        ).into(),
                        Expr::val(0.0).into(),
                    ])).from(InventoryTransactions).join(JoinType::InnerJoin, Products,
                        Expr::col((Products, products::Column::Id)).equals((InventoryTransactions, inventory_transactions::Column::ProductId))).cond_where(
                        Expr::cust("inventory_transactions.created_at >= strftime('%Y-%m-01', CURRENT_DATE)").and(Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)).eq("IN"))
                    ).to_owned()
                )),
            ))
        ), Alias::new("current_expenses")).expr_as(Expr::expr(
            Expr::expr(SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select().expr(Func::coalesce([
                        Func::sum(
                            Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)).mul(Expr::col((Products, products::Column::PurchasePrice)))
                        ).into(),
                        Expr::val(0.0).into(),
                    ])).from(InventoryTransactions).join(JoinType::InnerJoin, Products,
                        Expr::col((Products, products::Column::Id)).equals((InventoryTransactions, inventory_transactions::Column::ProductId))).cond_where(
                        Expr::cust("inventory_transactions.created_at < strftime('%Y-%m-01', CURRENT_DATE)").and(Expr::cust("inventory_transactions.created_at >= strftime('%Y-%m-01', CURRENT_DATE, '-1 month')")).and(Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)).eq("IN"))
                    ).to_owned()
                )),
            ))
        ), Alias::new("last_month_expenses")).to_owned().build(SqliteQueryBuilder);

        let res: Finiacialmetrics = Finiacialmetrics::find_by_statement(
            Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
        )
        .one(db)
        .await?
        .ok_or(DbErr::Custom("No data found".to_string()))?;

        let current_net_profit = res.current_revenue - res.current_expenses;
        let last_month_net_profit = res.last_month_revenue - res.last_month_expenses;
        let revenue_growth_percentage = if res.last_month_revenue != 0.0 {
            (res.current_revenue - res.last_month_revenue) / res.last_month_revenue
        } else {
            0.0
        };
        let expenses_growth_percentage = if res.last_month_expenses != 0.0 {
            (res.current_expenses - res.last_month_expenses) / res.last_month_expenses
        } else {
            0.0
        };
        let net_profit_growth_percentage = if last_month_net_profit != 0.0 {
            (current_net_profit - last_month_net_profit) / last_month_net_profit
        } else {
            0.0
        };

        Ok(FinancialMetricsResponse {
            current_revenue: res.current_revenue,
            last_month_revenue: res.last_month_revenue,
            current_expenses: res.current_expenses,
            last_month_expenses: res.last_month_expenses,
            current_net_profit,
            last_month_net_profit,
            revenue_growth_percentage,
            expenses_growth_percentage,
            net_profit_growth_percentage,
        })
    }
}
