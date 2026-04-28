use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Client::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Client::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(Client::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Client::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Client::Phone).string())
                    .col(ColumnDef::new(Client::Email).string())
                    .col(ColumnDef::new(Client::Address).string())
                    .col(ColumnDef::new(Client::Image).string())
                    .col(ColumnDef::new(Client::Ice).string())
                    .col(ColumnDef::new(Client::IfNumber).string())
                    .col(ColumnDef::new(Client::Rc).string())
                    .col(ColumnDef::new(Client::Patente).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Product::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Product::Description).string())
                    .col(
                        ColumnDef::new(Product::PurchasePrice)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(Product::SellingPrice)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(Product::MinQuantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(Product::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InventoryTransaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InventoryTransaction::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InventoryTransaction::TransactionType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InventoryTransaction::Quantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(InventoryTransaction::ProductId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InventoryTransaction::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_transaction_product_id")
                            .from(InventoryTransaction::Table, InventoryTransaction::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InventoryTransaction::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Client {
    #[sea_orm(iden = "clients")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "full_name")]
    Fullname,
    #[sea_orm(iden = "phone_number")]
    Phone,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "address")]
    Address,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "ice")]
    Ice,
    #[sea_orm(iden = "if_number")]
    IfNumber,
    #[sea_orm(iden = "rc")]
    Rc,
    #[sea_orm(iden = "patente")]
    Patente,
}

#[derive(DeriveIden)]
enum Product {
    #[sea_orm(iden = "products")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "purchase_price")]
    PurchasePrice,
    #[sea_orm(iden = "selling_price")]
    SellingPrice,
    #[sea_orm(iden = "min_quantity")]
    MinQuantity,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
}

#[derive(DeriveIden)]
enum InventoryTransaction {
    #[sea_orm(iden = "inventory_transactions")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "transaction_type")]
    TransactionType,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
