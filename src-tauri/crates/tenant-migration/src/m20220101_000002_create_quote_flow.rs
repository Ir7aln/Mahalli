use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Quote::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Quote::ClientId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_client_id")
                            .from(Quote::Table, Quote::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Quote::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Quote::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Quote::Identifier).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuoteItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QuoteItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(QuoteItem::Quantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(QuoteItem::ProductId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_item_product_id")
                            .from(QuoteItem::Table, QuoteItem::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(QuoteItem::QuoteId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_item_quote_id")
                            .from(QuoteItem::Table, QuoteItem::QuoteId)
                            .to(Quote::Table, Quote::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(QuoteItem::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuoteItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Quote {
    #[sea_orm(iden = "quotes")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
enum QuoteItem {
    #[sea_orm(iden = "quote_items")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "quote_id")]
    QuoteId,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
enum Client {
    #[sea_orm(iden = "clients")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}

#[derive(DeriveIden)]
enum Product {
    #[sea_orm(iden = "products")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}
