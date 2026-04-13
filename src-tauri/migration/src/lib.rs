pub use sea_orm_migration::prelude::*;

mod utils;

mod m20220101_000001_init_;
mod m20240216_201704_seed_init_;
mod m20260413_000001_invoice_items;
mod m20241020_121048_templates;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    // #[cfg(debug_assertions)]
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init_::Migration),
            Box::new(m20240216_201704_seed_init_::Migration),
            Box::new(m20241020_121048_templates::Migration),
            Box::new(m20260413_000001_invoice_items::Migration),
        ]
    }
    //
    // #[cfg(not(debug_assertions))]
    // fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    //     vec![
    //         Box::new(m20220101_000001_init_::Migration),
    //     ]
    // }
}
