use super::dto::*;
use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement,
    },
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients::{self, Entity as Clients},
    inventory_transactions::{self, Entity as InventoryTransactions},
    invoices::{self, Entity as Invoices},
    order_items::{self, Entity as OrderItems},
    orders::{self, Entity as Orders},
    products::{self, Entity as Products},
    suppliers::{self, Entity as Suppliers},
};

pub struct DashboardService;

impl DashboardService {
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
                        Expr::col((OrderItems, order_items::Column::Price)),
                        Expr::col((Products, products::Column::PurchasePrice)),
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
                        Expr::col((Orders, orders::Column::Status)),
                        Expr::expr("PENDING"),
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
        let (sql, values) = Query::select()
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select().expr(Func::coalesce([
                            Expr::expr(Func::sum(
                                Expr::case(
                                    Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                    Expr::col((
                                        InventoryTransactions,
                                        inventory_transactions::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .case(
                                    Expr::col((Invoices, invoices::Column::Status))
                                        .eq("PARTIALLY_PAID"),
                                    Expr::col((Invoices, invoices::Column::PaidAmount)),
                                )
                                .finally(Expr::val(0)),
                            )),
                            Expr::val(0.0),
                        ]))
                        .from(InventoryTransactions)
                        .join(
                            JoinType::InnerJoin,
                            OrderItems,
                            Expr::col((OrderItems, order_items::Column::InventoryId))
                                .equals((InventoryTransactions, inventory_transactions::Column::Id)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Orders,
                            Expr::col((Orders, orders::Column::Id))
                                .equals((OrderItems, order_items::Column::OrderId)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Invoices,
                            Expr::col((Invoices, invoices::Column::OrderId))
                                .equals((Orders, orders::Column::Id)),
                        )
                        .cond_where(Expr::cust(
                            "invoices.created_at >= strftime('%Y-%m-01', CURRENT_DATE)",
                        )
                        .and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec![
                            "PAID",
                            "PARTIALLY_PAID",
                        ]))
                        .and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false)))
                        .to_owned(),
                    )),
                )),
                Alias::new("current_revenue"),
            )
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select().expr(Func::coalesce([
                            Expr::expr(Func::sum(
                                Expr::case(
                                    Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                    Expr::col((
                                        InventoryTransactions,
                                        inventory_transactions::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .case(
                                    Expr::col((Invoices, invoices::Column::Status))
                                        .eq("PARTIALLY_PAID"),
                                    Expr::col((Invoices, invoices::Column::PaidAmount)),
                                )
                                .finally(Expr::val(0)),
                            )),
                            Expr::val(0.0),
                        ]))
                        .from(InventoryTransactions)
                        .join(
                            JoinType::InnerJoin,
                            OrderItems,
                            Expr::col((OrderItems, order_items::Column::InventoryId))
                                .equals((InventoryTransactions, inventory_transactions::Column::Id)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Orders,
                            Expr::col((Orders, orders::Column::Id))
                                .equals((OrderItems, order_items::Column::OrderId)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Invoices,
                            Expr::col((Invoices, invoices::Column::OrderId))
                                .equals((Orders, orders::Column::Id)),
                        )
                        .cond_where(Expr::cust(
                            "invoices.created_at >= date('now', '-1 month', 'start of month') AND invoices.created_at < date('now', 'start of month')",
                        )
                        .and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec![
                            "PAID",
                            "PARTIALLY_PAID",
                        ]))
                        .and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false)))
                        .to_owned(),
                    )),
                )),
                Alias::new("last_month_revenue"),
            )
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select().expr(Func::coalesce([
                            Expr::expr(Func::sum(
                                Expr::case(
                                    Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                    Expr::col((
                                        InventoryTransactions,
                                        inventory_transactions::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .case(
                                    Expr::col((Invoices, invoices::Column::Status))
                                        .eq("PARTIALLY_PAID"),
                                    Expr::col((Invoices, invoices::Column::PaidAmount)),
                                )
                                .finally(Expr::val(0)),
                            )),
                            Expr::val(0.0),
                        ]))
                        .from(InventoryTransactions)
                        .join(
                            JoinType::InnerJoin,
                            OrderItems,
                            Expr::col((OrderItems, order_items::Column::InventoryId))
                                .equals((InventoryTransactions, inventory_transactions::Column::Id)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Orders,
                            Expr::col((Orders, orders::Column::Id))
                                .equals((OrderItems, order_items::Column::OrderId)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Invoices,
                            Expr::col((Invoices, invoices::Column::OrderId))
                                .equals((Orders, orders::Column::Id)),
                        )
                        .cond_where(Expr::cust(
                            "invoices.created_at >= strftime('%Y-%m-01', date('now', '-1 month')) AND invoices.created_at < strftime('%Y-%m-01', CURRENT_DATE)",
                        )
                        .and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec![
                            "PAID",
                            "PARTIALLY_PAID",
                        ]))
                        .and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false)))
                        .to_owned(),
                    )),
                )),
                Alias::new("current_expenses"),
            )
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select().expr(Func::coalesce([
                            Expr::expr(Func::sum(
                                Expr::case(
                                    Expr::col((Invoices, invoices::Column::Status)).eq("PAID"),
                                    Expr::col((
                                        InventoryTransactions,
                                        inventory_transactions::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .case(
                                    Expr::col((Invoices, invoices::Column::Status))
                                        .eq("PARTIALLY_PAID"),
                                    Expr::col((Invoices, invoices::Column::PaidAmount)),
                                )
                                .finally(Expr::val(0)),
                            )),
                            Expr::val(0.0),
                        ]))
                        .from(InventoryTransactions)
                        .join(
                            JoinType::InnerJoin,
                            OrderItems,
                            Expr::col((OrderItems, order_items::Column::InventoryId))
                                .equals((InventoryTransactions, inventory_transactions::Column::Id)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Orders,
                            Expr::col((Orders, orders::Column::Id))
                                .equals((OrderItems, order_items::Column::OrderId)),
                        )
                        .join(
                            JoinType::InnerJoin,
                            Invoices,
                            Expr::col((Invoices, invoices::Column::OrderId))
                                .equals((Orders, orders::Column::Id)),
                        )
                        .cond_where(Expr::cust(
                            "invoices.created_at >= date('now', '-1 month', 'start of month') AND invoices.created_at < date('now', 'start of month')",
                        )
                        .and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec![
                            "PAID",
                            "PARTIALLY_PAID",
                        ]))
                        .and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false)))
                        .to_owned(),
                    )),
                )),
                Alias::new("last_month_expenses"),
            )
            .to_owned()
            .build(SqliteQueryBuilder);

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
        } * 100.0;
        let expenses_growth_percentage = if res.last_month_expenses != 0.0 {
            (res.current_expenses - res.last_month_expenses) / res.last_month_expenses
        } else {
            0.0
        } * 100.0;
        let net_profit_growth_percentage = if last_month_net_profit != 0.0 {
            (current_net_profit - last_month_net_profit) / last_month_net_profit
        } else {
            0.0
        } * 100.0;
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
