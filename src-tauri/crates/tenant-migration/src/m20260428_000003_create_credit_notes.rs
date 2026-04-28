use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CreditNotes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditNotes::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::InvoiceId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::ClientId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::Identifier)
                            .string()
                            .unique_key()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::Reason)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(CreditNotes::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_credit_notes_invoice_id")
                            .from(CreditNotes::Table, CreditNotes::InvoiceId)
                            .to(Invoices::Table, Invoices::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_credit_notes_client_id")
                            .from(CreditNotes::Table, CreditNotes::ClientId)
                            .to(Clients::Table, Clients::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CreditNoteItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditNoteItems::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CreditNoteItems::CreditNoteId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNoteItems::ProductId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNoteItems::Quantity)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNoteItems::Price)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditNoteItems::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_credit_note_items_credit_note_id")
                            .from(CreditNoteItems::Table, CreditNoteItems::CreditNoteId)
                            .to(CreditNotes::Table, CreditNotes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_credit_note_items_product_id")
                            .from(CreditNoteItems::Table, CreditNoteItems::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditNoteItems::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CreditNotes::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum CreditNotes {
    #[sea_orm(iden = "credit_notes")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "identifier")]
    Identifier,
    #[sea_orm(iden = "reason")]
    Reason,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
enum CreditNoteItems {
    #[sea_orm(iden = "credit_note_items")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "credit_note_id")]
    CreditNoteId,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
enum Invoices {
    #[sea_orm(iden = "invoices")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}

#[derive(DeriveIden)]
enum Clients {
    #[sea_orm(iden = "clients")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}

#[derive(DeriveIden)]
enum Products {
    #[sea_orm(iden = "products")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
}
