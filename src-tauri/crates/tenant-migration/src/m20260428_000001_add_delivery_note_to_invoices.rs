use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Invoices::DeliveryNoteId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_invoices_delivery_note_id")
                    .from(Invoices::Table, Invoices::DeliveryNoteId)
                    .to(DeliveryNotes::Table, DeliveryNotes::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_invoices_delivery_note_id")
                    .table(Invoices::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(Invoices::DeliveryNoteId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Invoices {
    Table,
    DeliveryNoteId,
}

#[derive(DeriveIden)]
enum DeliveryNotes {
    Table,
    Id,
}
