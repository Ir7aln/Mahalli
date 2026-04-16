use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectTransaction {
    pub created_at: String,
    pub price: f64,
    pub quantity: f64,
    pub transaction_type: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectTops {
    pub full_name: String,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectTopProducts {
    pub name: String,
    pub quantity: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectStatusCount {
    pub status: String,
    pub status_count: i64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectRevenue {
    pub current_revenue: f64,
    pub last_month_revenue: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct SelectExpenses {
    pub current_expenses: f64,
    pub last_month_expenses: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct Finiacialmetrics {
    pub current_revenue: f64,
    pub last_month_revenue: f64,
    pub current_expenses: f64,
    pub last_month_expenses: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct InventoryStatsResponse {
    pub total_items: i64,
    pub total_value: f64,
    pub low_stock_count: i64,
    pub out_of_stock_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct StatusCountResponse {
    pub orders: Vec<SelectStatusCount>,
    pub invoices: Vec<SelectStatusCount>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct FinancialMetricsResponse {
    pub current_revenue: f64,
    pub last_month_revenue: f64,
    pub current_expenses: f64,
    pub last_month_expenses: f64,
    pub current_net_profit: f64,
    pub last_month_net_profit: f64,
    pub revenue_growth_percentage: f64,
    pub expenses_growth_percentage: f64,
    pub net_profit_growth_percentage: f64,
}
