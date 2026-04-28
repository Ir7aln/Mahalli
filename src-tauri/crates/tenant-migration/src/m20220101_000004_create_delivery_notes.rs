use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeliveryNote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeliveryNote::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DeliveryNote::OrderId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_delivery_note_order_id")
                            .from(DeliveryNote::Table, DeliveryNote::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(DeliveryNote::ClientId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_delivery_note_client_id")
                            .from(DeliveryNote::Table, DeliveryNote::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(DeliveryNote::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(DeliveryNote::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(DeliveryNote::Identifier).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DeliveryNoteItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeliveryNoteItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DeliveryNoteItem::DeliveryNoteId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_delivery_note_item_delivery_note_id")
                            .from(DeliveryNoteItem::Table, DeliveryNoteItem::DeliveryNoteId)
                            .to(DeliveryNote::Table, DeliveryNote::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(DeliveryNoteItem::ProductId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_delivery_note_item_product_id")
                            .from(DeliveryNoteItem::Table, DeliveryNoteItem::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(DeliveryNoteItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(DeliveryNoteItem::Quantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(DeliveryNoteItem::CreatedAt)
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
            .drop_table(Table::drop().table(DeliveryNoteItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DeliveryNote::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum DeliveryNote {
    #[sea_orm(iden = "delivery_notes")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "order_id")]
    OrderId,
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
enum DeliveryNoteItem {
    #[sea_orm(iden = "delivery_note_items")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "delivery_note_id")]
    DeliveryNoteId,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
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
