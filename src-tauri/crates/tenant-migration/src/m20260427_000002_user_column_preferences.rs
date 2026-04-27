use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserColumnPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserColumnPreferences::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserColumnPreferences::Page)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserColumnPreferences::VisibleColumns)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserColumnPreferences::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserColumnPreferences::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(UserColumnPreferences::Table)
                    .col(UserColumnPreferences::Page)
                    .name("idx_user_column_preferences_page")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(UserColumnPreferences::Table)
                    .name("idx_user_column_preferences_page")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserColumnPreferences::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserColumnPreferences {
    #[sea_orm(iden = "user_column_preferences")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "page")]
    Page,
    #[sea_orm(iden = "visible_columns")]
    VisibleColumns,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
