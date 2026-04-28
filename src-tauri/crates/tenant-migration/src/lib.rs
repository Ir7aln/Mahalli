pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_core_entities;
mod m20220101_000002_create_quote_flow;
mod m20220101_000003_create_order_flow;
mod m20220101_000004_create_delivery_notes;
mod m20220101_000005_create_invoices;
mod m20220101_000006_create_indexes_and_triggers;
mod m20241020_121048_templates;
mod m20260427_000002_user_column_preferences;
mod m20260428_000003_create_credit_notes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_core_entities::Migration),
            Box::new(m20220101_000002_create_quote_flow::Migration),
            Box::new(m20220101_000003_create_order_flow::Migration),
            Box::new(m20220101_000004_create_delivery_notes::Migration),
            Box::new(m20220101_000005_create_invoices::Migration),
            Box::new(m20220101_000006_create_indexes_and_triggers::Migration),
            Box::new(m20241020_121048_templates::Migration),
            Box::new(m20260427_000002_user_column_preferences::Migration),
            Box::new(m20260428_000003_create_credit_notes::Migration),
        ]
    }
}
