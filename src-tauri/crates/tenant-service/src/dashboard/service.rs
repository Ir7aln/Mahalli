use super::dto::*;
use sea_orm::{
    sea_query::{Alias, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    inventory_transactions, invoice_payments, invoices, order_items, orders, prelude::*, products,
};

pub struct DashboardService;

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

fn revenue_expr(start: &'static str, end: Option<&'static str>) -> SimpleExpr {
    let mut condition = Expr::cust(start)
        .and(Expr::col((Invoices, invoices::Column::Status)).is_in(vec!["PAID", "PARTIALLY_PAID"]))
        .and(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
        .and(
            Expr::col((Orders, orders::Column::Status))
                .eq("CANCELLED")
                .not(),
        );

    if let Some(end) = end {
        condition = condition.and(Expr::cust(end));
    }

    Expr::expr(SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .expr(Func::coalesce([
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
                            Expr::col((Invoices, invoices::Column::Status)).eq("PARTIALLY_PAID"),
                            invoice_paid_amount_expr(),
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
                .cond_where(condition)
                .to_owned(),
        )),
    ))
}

fn expense_expr(start: &'static str, end: Option<&'static str>) -> SimpleExpr {
    let mut condition = Expr::cust(start)
        .and(
            Expr::col((
                InventoryTransactions,
                inventory_transactions::Column::TransactionType,
            ))
            .eq("IN"),
        )
        .and(
            Expr::col((
                InventoryTransactions,
                inventory_transactions::Column::IsVoid,
            ))
            .eq(false),
        );

    if let Some(end) = end {
        condition = condition.and(Expr::cust(end));
    }

    Expr::expr(SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .expr(Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::expr(Func::coalesce([
                            Expr::col((
                                InventoryTransactions,
                                inventory_transactions::Column::UnitPrice,
                            )),
                            Expr::col((Products, products::Column::PurchasePrice)),
                            Expr::val(0.0),
                        ]))
                        .mul(Expr::col((
                            InventoryTransactions,
                            inventory_transactions::Column::Quantity,
                        ))),
                    )),
                    Expr::val(0.0),
                ]))
                .from(InventoryTransactions)
                .join(
                    JoinType::InnerJoin,
                    Products,
                    Expr::col((Products, products::Column::Id)).equals((
                        InventoryTransactions,
                        inventory_transactions::Column::ProductId,
                    )),
                )
                .cond_where(condition)
                .to_owned(),
        )),
    ))
}

impl DashboardService {
    pub async fn list_financial_metrics(db: &DbConn) -> Result<FinancialMetricsResponse, DbErr> {
        let current_month_start = "invoices.created_at >= strftime('%Y-%m-01', CURRENT_DATE)";
        let last_month_start = "invoices.created_at >= date('now', '-1 month', 'start of month')";
        let last_month_end = "invoices.created_at < date('now', 'start of month')";
        let current_expense_start =
            "inventory_transactions.created_at >= strftime('%Y-%m-01', CURRENT_DATE)";
        let last_expense_start =
            "inventory_transactions.created_at >= date('now', '-1 month', 'start of month')";
        let last_expense_end = "inventory_transactions.created_at < date('now', 'start of month')";

        let (sql, values) = Query::select()
            .expr_as(
                revenue_expr(current_month_start, None),
                Alias::new("current_revenue"),
            )
            .expr_as(
                revenue_expr(last_month_start, Some(last_month_end)),
                Alias::new("last_month_revenue"),
            )
            .expr_as(
                expense_expr(current_expense_start, None),
                Alias::new("current_expenses"),
            )
            .expr_as(
                expense_expr(last_expense_start, Some(last_expense_end)),
                Alias::new("last_month_expenses"),
            )
            .to_owned()
            .build(SqliteQueryBuilder);

        let res: FinancialMetricsQueryResult = FinancialMetricsQueryResult::find_by_statement(
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
