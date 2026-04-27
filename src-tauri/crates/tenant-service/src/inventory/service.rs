use super::dto::*;
use sea_orm::{
    sea_query::{Alias, Cond, Expr, Func, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    inventory_transactions::{
        self, ActiveModel as InventoryActiveModel, Entity as InventoryTransactions,
    },
    invoice_items::{self, Entity as InvoiceItems},
    invoices::{self, Entity as Invoices},
    order_items::{self, Entity as OrderItems},
    orders::{self, Entity as Orders},
    products::{self, Entity as Products},
    quotes::{self, Entity as Quotes},
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn inventory_price_expr() -> sea_orm::sea_query::SimpleExpr {
    Expr::expr(Func::coalesce([
        Expr::col((OrderItems, order_items::Column::Price)),
        Expr::col((Products, products::Column::PurchasePrice)),
    ]))
}

pub struct InventoryService;

impl InventoryService {
    pub async fn list_inventory(
        db: &DbConn,
        args: ListInventoryArgs,
    ) -> Result<InventoryResponse, DbErr> {
        let count = InventoryTransactions::find()
            .join(JoinType::Join, inventory_transactions::Relation::Products.def())
            .join(JoinType::LeftJoin, inventory_transactions::Relation::OrderItems.def())
            .join(JoinType::LeftJoin, inventory_transactions::Relation::InvoiceItems.def())
            .join(JoinType::LeftJoin, invoice_items::Relation::Invoices.def())
            .join(JoinType::LeftJoin, order_items::Relation::Orders.def())
            .filter(
                Cond::all()
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Orders, orders::Column::Status)),
                            Expr::expr("PENDING"),
                        ]))
                        .eq("CANCELLED")
                        .not(),
                    )
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::Status)),
                            Expr::expr("PENDING"),
                        ]))
                        .eq("CANCELLED")
                        .not(),
                    )
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Orders, orders::Column::IsDeleted)),
                            Expr::expr(false),
                        ]))
                        .eq(false),
                    ),
            )
            .apply_if(Some(args.search.clone()), |query, v| {
                query.filter(Expr::col((Products, products::Column::Name)).like(format!("%{}%", v)))
            })
            .apply_if(args.transaction_type.clone(), |query, v| {
                query.filter(
                    Expr::col((
                        InventoryTransactions,
                        inventory_transactions::Column::TransactionType,
                    ))
                    .eq(v),
                )
            })
            .apply_if(args.created_from.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', inventory_transactions.created_at) >= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.created_to.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', inventory_transactions.created_at) <= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.quantity_min, |query, value| {
                query.filter(
                    Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))
                        .gte(value),
                )
            })
            .apply_if(args.quantity_max, |query, value| {
                query.filter(
                    Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))
                        .lte(value),
                )
            })
            .apply_if(args.price_min, |query, value| query.filter(inventory_price_expr().gte(value)))
            .apply_if(args.price_max, |query, value| query.filter(inventory_price_expr().lte(value)))
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(InventoryTransactions)
            .exprs([
                Expr::col((InventoryTransactions, inventory_transactions::Column::Id)),
                Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity)),
                Expr::col((Products, products::Column::Name)),
                Expr::col((InventoryTransactions, inventory_transactions::Column::TransactionType)),
                Expr::col((InventoryTransactions, inventory_transactions::Column::CreatedAt)),
            ])
            .expr_as(
                Expr::col((Orders, orders::Column::Id)),
                Alias::new("order_id"),
            )
            .expr_as(
                Expr::col((Orders, orders::Column::Identifier)),
                Alias::new("order_identifier"),
            )
            .expr_as(
                Expr::col((Invoices, invoices::Column::Id)),
                Alias::new("invoice_id"),
            )
            .expr_as(
                Expr::col((Invoices, invoices::Column::Identifier)),
                Alias::new("invoice_identifier"),
            )
            .expr_as(
                Expr::col((Quotes, quotes::Column::Id)),
                Alias::new("quote_id"),
            )
            .expr_as(
                Expr::col((Quotes, quotes::Column::Identifier)),
                Alias::new("quote_identifier"),
            )
            .expr_as(inventory_price_expr(), Alias::new("price"))
            .join(
                JoinType::Join,
                Products,
                Expr::col((Products, products::Column::Id))
                    .equals((InventoryTransactions, inventory_transactions::Column::ProductId)),
            )
            .join(
                JoinType::LeftJoin,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::InventoryId)).equals((
                    InventoryTransactions,
                    inventory_transactions::Column::Id,
                )),
            )
            .join(
                JoinType::LeftJoin,
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InventoryId)).equals((
                    InventoryTransactions,
                    inventory_transactions::Column::Id,
                )),
            )
            .join(
                JoinType::LeftJoin,
                Invoices,
                Expr::col((Invoices, invoices::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InvoiceId)),
            )
            .join(
                JoinType::LeftJoin,
                Orders,
                Expr::col((Orders, orders::Column::Id)).equals((
                    OrderItems,
                    order_items::Column::OrderId,
                )),
            )
            .join(
                JoinType::LeftJoin,
                Quotes,
                Expr::col((Quotes, quotes::Column::Id))
                    .equals((Orders, orders::Column::QuoteId)),
            )
            .cond_where(
                Cond::all().add(
                    Expr::expr(Func::coalesce([
                        Expr::col((Orders, orders::Column::Status)),
                        Expr::expr("PENDING"),
                    ]))
                    .eq("CANCELLED")
                    .not(),
                )
                .add(
                    Expr::expr(Func::coalesce([
                        Expr::col((Invoices, invoices::Column::Status)),
                        Expr::expr("PENDING"),
                    ]))
                    .eq("CANCELLED")
                    .not(),
                )
                .add(
                    Expr::expr(Func::coalesce([
                        Expr::col((Orders, orders::Column::IsDeleted)),
                        Expr::expr(false),
                    ]))
                    .eq(false),
                ),
            )
            .and_where(Expr::col((Products, products::Column::Name)).like(format!("%{}%", args.search)))
            .conditions(
                args.transaction_type.clone().is_some(),
                |x| {
                    x.and_where(
                        Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::TransactionType,
                        ))
                        .eq(args.transaction_type),
                    );
                },
                |_| {},
            )
            .conditions(
                args.created_from.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', inventory_transactions.created_at) >= strftime('%Y-%m-%d', ?)",
                        args.created_from,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.created_to.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', inventory_transactions.created_at) <= strftime('%Y-%m-%d', ?)",
                        args.created_to,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.quantity_min.is_some(),
                |x| {
                    x.and_where(
                        Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))
                            .gte(args.quantity_min),
                    );
                },
                |_| {},
            )
            .conditions(
                args.quantity_max.is_some(),
                |x| {
                    x.and_where(
                        Expr::col((InventoryTransactions, inventory_transactions::Column::Quantity))
                            .lte(args.quantity_max),
                    );
                },
                |_| {},
            )
            .conditions(
                args.price_min.is_some(),
                |x| {
                    x.and_where(inventory_price_expr().gte(args.price_min));
                },
                |_| {},
            )
            .conditions(
                args.price_max.is_some(),
                |x| {
                    x.and_where(inventory_price_expr().lte(args.price_max));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);
        match args.sort.as_deref() {
            Some("name") => query.order_by(
                (Products, products::Column::Name),
                requested_order(args.direction.as_deref()),
            ),
            Some("price") => query.order_by_expr(
                Expr::cust("price"),
                requested_order(args.direction.as_deref()),
            ),
            Some("quantity") => query.order_by(
                (
                    InventoryTransactions,
                    inventory_transactions::Column::Quantity,
                ),
                requested_order(args.direction.as_deref()),
            ),
            Some("transaction_type") => query.order_by(
                (
                    InventoryTransactions,
                    inventory_transactions::Column::TransactionType,
                ),
                requested_order(args.direction.as_deref()),
            ),
            Some("created_at") => query.order_by(
                (
                    InventoryTransactions,
                    inventory_transactions::Column::CreatedAt,
                ),
                requested_order(args.direction.as_deref()),
            ),
            _ => query.order_by(
                (
                    InventoryTransactions,
                    inventory_transactions::Column::CreatedAt,
                ),
                Order::Desc,
            ),
        };
        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

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

    pub async fn create_inventory(db: &DbConn, transaction: NewInventory) -> Result<String, DbErr> {
        let in_transaction = InventoryActiveModel {
            transaction_type: ActiveValue::Set(transaction.transaction_type),
            quantity: ActiveValue::Set(transaction.quantity),
            product_id: ActiveValue::Set(transaction.product_id),
            ..Default::default()
        };
        match in_transaction.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_inventory(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let inventory = InventoryTransactions::find_by_id(id).one(db).await?;
        match inventory {
            Some(inventory) => {
                let transaction = inventory.delete(db).await?;
                Ok(transaction.rows_affected)
            }
            None => Ok(0),
        }
    }
}
