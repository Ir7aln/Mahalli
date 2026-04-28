use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Completed => "COMPLETED",
            OrderStatus::Cancelled => "CANCELLED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(OrderStatus::Pending),
            "COMPLETED" => Some(OrderStatus::Completed),
            "CANCELLED" => Some(OrderStatus::Cancelled),
            _ => None,
        }
    }

    pub fn is_valid_transition(&self, next: &OrderStatus) -> bool {
        match (self, next) {
            (OrderStatus::Cancelled, OrderStatus::Cancelled) => true,
            (OrderStatus::Cancelled, _) => false,
            (OrderStatus::Completed, OrderStatus::Completed) => true,
            (OrderStatus::Completed, _) => false,
            (OrderStatus::Pending, OrderStatus::Completed) => true,
            (_, OrderStatus::Cancelled) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceStatus {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "FINALIZED")]
    Finalized,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "PARTIALLY_PAID")]
    PartiallyPaid,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "DRAFT",
            InvoiceStatus::Finalized => "FINALIZED",
            InvoiceStatus::Paid => "PAID",
            InvoiceStatus::PartiallyPaid => "PARTIALLY_PAID",
            InvoiceStatus::Cancelled => "CANCELLED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DRAFT" => Some(InvoiceStatus::Draft),
            "FINALIZED" => Some(InvoiceStatus::Finalized),
            "PAID" => Some(InvoiceStatus::Paid),
            "PARTIALLY_PAID" => Some(InvoiceStatus::PartiallyPaid),
            "CANCELLED" => Some(InvoiceStatus::Cancelled),
            _ => None,
        }
    }

    pub fn is_valid_transition(&self, next: &InvoiceStatus) -> bool {
        match (self, next) {
            (InvoiceStatus::Cancelled, InvoiceStatus::Cancelled) => true,
            (InvoiceStatus::Cancelled, _) => false,
            (InvoiceStatus::Paid, InvoiceStatus::Paid) => true,
            (InvoiceStatus::Paid, _) => false,
            (InvoiceStatus::Draft, InvoiceStatus::Finalized) => true,
            (InvoiceStatus::Draft, InvoiceStatus::Paid) => true,
            (InvoiceStatus::Draft, InvoiceStatus::PartiallyPaid) => true,
            (InvoiceStatus::Finalized, InvoiceStatus::Finalized) => true,
            (InvoiceStatus::PartiallyPaid, InvoiceStatus::Paid) => true,
            (_, InvoiceStatus::Cancelled) => true,
            _ => false,
        }
    }
}
