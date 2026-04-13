use sea_orm_migration::prelude::*;

use crate::m20220101_000001_init_::{
    Invoice,
    Product,
    InventoryTransaction,
};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InvoiceItem::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InvoiceItem::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(InvoiceItem::InvoiceId).string().not_null())
                    .col(ColumnDef::new(InvoiceItem::ProductId).string().not_null())
                    .col(ColumnDef::new(InvoiceItem::Price).double().not_null().default(0))
                    .col(ColumnDef::new(InvoiceItem::Quantity).double().not_null().default(0))
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

        manager
            .create_index(
                Index::create()
                    .name("idx_invoice_items_invoice_id")
                    .table(InvoiceItem::Table)
                    .col(InvoiceItem::InvoiceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InvoiceItem::Table).to_owned())
            .await
    }
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
