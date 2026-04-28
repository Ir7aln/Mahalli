pub use sea_orm_migration::prelude::*;

mod m20220101_000001_init_;
mod m20241020_121048_templates;
mod m20260427_000002_user_column_preferences;
mod m20260428_000001_add_delivery_note_to_invoices;
mod m20260428_000002_add_finalized_at_to_invoices;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init_::Migration),
            Box::new(m20241020_121048_templates::Migration),
            Box::new(m20260427_000002_user_column_preferences::Migration),
            Box::new(m20260428_000001_add_delivery_note_to_invoices::Migration),
            Box::new(m20260428_000002_add_finalized_at_to_invoices::Migration),
        ]
    }
}
