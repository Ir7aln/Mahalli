pub use sea_orm_migration::prelude::*;

mod m20260416_000001_create_databases;
mod m20260428_000001_seller_profile;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260416_000001_create_databases::Migration),
            Box::new(m20260428_000001_seller_profile::Migration),
        ]
    }
}
