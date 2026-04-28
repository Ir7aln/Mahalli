use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invoice::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invoice::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Invoice::ClientId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_client_id")
                            .from(Invoice::Table, Invoice::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Invoice::OrderId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_order_id")
                            .from(Invoice::Table, Invoice::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(
                        ColumnDef::new(Invoice::DeliveryNoteId)
                            .string()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_delivery_note_id")
                            .from(Invoice::Table, Invoice::DeliveryNoteId)
                            .to(DeliveryNote::Table, DeliveryNote::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(
                        ColumnDef::new(Invoice::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Invoice::Status).string().not_null())
                    .col(ColumnDef::new(Invoice::Identifier).string())
                    .col(
                        ColumnDef::new(Invoice::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Invoice::FinalizedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InvoicePayment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InvoicePayment::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InvoicePayment::InvoiceId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvoicePayment::PaymentDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(InvoicePayment::Description).string())
                    .col(
                        ColumnDef::new(InvoicePayment::Amount)
                            .double()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(InvoicePayment::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_payments_invoice_id")
                            .from(InvoicePayment::Table, InvoicePayment::InvoiceId)
                            .to(Invoice::Table, Invoice::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InvoiceItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InvoiceItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InvoiceItem::InvoiceId).string().not_null())
                    .col(ColumnDef::new(InvoiceItem::ProductId).string().not_null())
                    .col(
                        ColumnDef::new(InvoiceItem::Price)
                            .double()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(InvoiceItem::Quantity)
                            .double()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(InvoiceItem::InventoryId).string().null())
                    .col(
                        ColumnDef::new(InvoiceItem::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_items_invoice_id")
                            .from(InvoiceItem::Table, InvoiceItem::InvoiceId)
                            .to(Invoice::Table, Invoice::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_items_product_id")
                            .from(InvoiceItem::Table, InvoiceItem::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_items_inventory_id")
                            .from(InvoiceItem::Table, InvoiceItem::InventoryId)
                            .to(InventoryTransaction::Table, InventoryTransaction::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InvoiceItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(InvoicePayment::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Invoice::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Invoice {
    #[sea_orm(iden = "invoices")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "delivery_note_id")]
    DeliveryNoteId,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "finalized_at")]
    FinalizedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
enum InvoicePayment {
    #[sea_orm(iden = "invoice_payments")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
    #[sea_orm(iden = "payment_date")]
    PaymentDate,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "amount")]
    Amount,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
enum InvoiceItem {
    #[sea_orm(iden = "invoice_items")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
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
enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}

#[derive(DeriveIden)]
enum DeliveryNote {
    #[sea_orm(iden = "delivery_notes")]
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

#[derive(DeriveIden)]
enum InventoryTransaction {
    #[sea_orm(iden = "inventory_transactions")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}
