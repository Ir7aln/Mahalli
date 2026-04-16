use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Templates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Templates::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Templates::ValuesJson)
                            .string()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Templates::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Templates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Templates {
    #[sea_orm(iden = "templates")]
    Table,
    Id,
    #[sea_orm(iden = "values_json")]
    ValuesJson,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
