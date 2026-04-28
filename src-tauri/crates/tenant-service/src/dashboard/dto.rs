use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult, Type)]
pub struct FinancialMetricsQueryResult {
    pub current_revenue: f64,
    pub last_month_revenue: f64,
    pub current_expenses: f64,
    pub last_month_expenses: f64,
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
