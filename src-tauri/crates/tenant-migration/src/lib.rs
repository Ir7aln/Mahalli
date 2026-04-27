pub use sea_orm_migration::prelude::*;

mod m20220101_000001_init_;
mod m20241020_121048_templates;
mod m20260427_000002_user_column_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init_::Migration),
            Box::new(m20241020_121048_templates::Migration),
            Box::new(m20260427_000002_user_column_preferences::Migration),
        ]
    }
}
