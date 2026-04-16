use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Databases::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Databases::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Databases::Name).string().not_null())
                    .col(ColumnDef::new(Databases::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Databases::FileName).string().not_null().unique_key())
                    .col(ColumnDef::new(Databases::FilePath).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Databases::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Databases::CreatedFromDatabaseId).string())
                    .col(ColumnDef::new(Databases::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Databases::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Databases::LastOpenedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Databases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Databases {
    Table,
    Id,
    Name,
    Slug,
    FileName,
    FilePath,
    IsActive,
    CreatedFromDatabaseId,
    CreatedAt,
    UpdatedAt,
    LastOpenedAt,
}
