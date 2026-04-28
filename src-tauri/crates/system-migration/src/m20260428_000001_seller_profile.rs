use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SellerProfile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SellerProfile::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SellerProfile::LegalName).string().not_null())
                    .col(ColumnDef::new(SellerProfile::CommercialName).string())
                    .col(ColumnDef::new(SellerProfile::Address).string())
                    .col(ColumnDef::new(SellerProfile::City).string())
                    .col(ColumnDef::new(SellerProfile::PhoneNumber).string())
                    .col(ColumnDef::new(SellerProfile::Email).string())
                    .col(ColumnDef::new(SellerProfile::Ice).string())
                    .col(ColumnDef::new(SellerProfile::IfNumber).string())
                    .col(ColumnDef::new(SellerProfile::Rc).string())
                    .col(ColumnDef::new(SellerProfile::Patente).string())
                    .col(ColumnDef::new(SellerProfile::Logo).string())
                    .col(
                        ColumnDef::new(SellerProfile::DefaultCurrency)
                            .string()
                            .default("MAD"),
                    )
                    .col(
                        ColumnDef::new(SellerProfile::DefaultPaymentTermsDays)
                            .integer()
                            .default(30),
                    )
                    .col(ColumnDef::new(SellerProfile::InvoiceFooter).text())
                    .col(
                        ColumnDef::new(SellerProfile::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SellerProfile::UpdatedAt)
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
            .drop_table(Table::drop().table(SellerProfile::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum SellerProfile {
    #[sea_orm(iden = "seller_profile")]
    Table,
    Id,
    #[sea_orm(iden = "legal_name")]
    LegalName,
    #[sea_orm(iden = "commercial_name")]
    CommercialName,
    #[sea_orm(iden = "address")]
    Address,
    #[sea_orm(iden = "city")]
    City,
    #[sea_orm(iden = "phone_number")]
    PhoneNumber,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "ice")]
    Ice,
    #[sea_orm(iden = "if_number")]
    IfNumber,
    #[sea_orm(iden = "rc")]
    Rc,
    #[sea_orm(iden = "patente")]
    Patente,
    #[sea_orm(iden = "logo")]
    Logo,
    #[sea_orm(iden = "default_currency")]
    DefaultCurrency,
    #[sea_orm(iden = "default_payment_terms_days")]
    DefaultPaymentTermsDays,
    #[sea_orm(iden = "invoice_footer")]
    InvoiceFooter,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
