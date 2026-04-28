use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Order::ClientId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_client_id")
                            .from(Order::Table, Order::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Order::QuoteId).string().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_quote_id")
                            .from(Order::Table, Order::QuoteId)
                            .to(Quote::Table, Quote::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(
                        ColumnDef::new(Order::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Order::Status).string().not_null())
                    .col(ColumnDef::new(Order::Identifier).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrderItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(OrderItem::OrderId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_item_order_id")
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrderItem::InventoryId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_item_inventory_id")
                            .from(OrderItem::Table, OrderItem::InventoryId)
                            .to(InventoryTransaction::Table, InventoryTransaction::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrderItem::CreatedAt)
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
            .drop_table(Table::drop().table(OrderItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "quote_id")]
    QuoteId,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
enum OrderItem {
    #[sea_orm(iden = "order_items")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
    #[sea_orm(iden = "price")]
    Price,
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
enum Quote {
    #[sea_orm(iden = "quotes")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}

#[derive(DeriveIden)]
enum InventoryTransaction {
    #[sea_orm(iden = "inventory_transactions")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}
