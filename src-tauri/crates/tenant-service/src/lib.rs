pub mod clients;
pub mod dashboard;
pub mod inventory;
pub mod invoices;
pub mod orders;
pub mod products;
pub mod quotes;
mod seed;
pub mod suppliers;
pub mod templates;

pub use seed::*;

pub use sea_orm;
