use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_raw(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"PRAGMA journal_mode=WAL;"#,
        ))
        .await?;

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
                    .col(
                        ColumnDef::new(Client::IsArchived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Client::Phone).string())
                    .col(ColumnDef::new(Client::Email).string())
                    .col(ColumnDef::new(Client::Address).string())
                    .col(ColumnDef::new(Client::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Supplier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplier::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplier::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(Supplier::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Supplier::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Supplier::IsArchived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Supplier::Phone).string())
                    .col(ColumnDef::new(Supplier::Email).string())
                    .col(ColumnDef::new(Supplier::Address).string())
                    .col(ColumnDef::new(Supplier::Image).string())
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
                    .col(
                        ColumnDef::new(Product::IsArchived)
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
                        ColumnDef::new(Quote::IsArchived)
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
                    .to_owned(),
            )
            .await?;

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
                        ColumnDef::new(Order::IsArchived)
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
                    .to_owned(),
            )
            .await?;

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
                    .col(
                        ColumnDef::new(Invoice::PaidAmount)
                            .float()
                            .not_null()
                            .default(0),
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
                        ColumnDef::new(Invoice::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Invoice::IsArchived)
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

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Quote::Table)
                    .col(Quote::ClientId)
                    .name("idx_quote_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(QuoteItem::Table)
                    .col(QuoteItem::ProductId)
                    .name("idx_quote_item_product_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(OrderItem::Table)
                    .col(OrderItem::InventoryId)
                    .name("idx_order_item_inventory_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Order::Table)
                    .col(Order::ClientId)
                    .name("idx_order_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Invoice::Table)
                    .col(Invoice::ClientId)
                    .name("idx_invoice_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(InventoryTransaction::Table)
                    .col(InventoryTransaction::ProductId)
                    .name("idx_inventory_product_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(InventoryTransaction::Table)
                    .col(InventoryTransaction::TransactionType)
                    .name("idx_inventory_transaction_type")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Order::Table)
                    .col(Order::Status)
                    .name("idx_orders_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Invoice::Table)
                    .col(Invoice::Status)
                    .name("idx_invoices_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(InvoiceItem::Table)
                    .col(InvoiceItem::InvoiceId)
                    .name("idx_invoice_items_invoice_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Client::Table)
                    .col(Client::Fullname)
                    .name("idx_clients_fullname")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Supplier::Table)
                    .col(Supplier::Fullname)
                    .name("idx_suppliers_fullname")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Product::Table)
                    .col(Product::Name)
                    .name("idx_products_name")
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        let i_identifier_generator = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
                CREATE TRIGGER IF NOT EXISTS invoice_identifier_generator
                AFTER INSERT ON invoices 
                BEGIN
                    UPDATE invoices 
                    SET identifier = (
                        WITH current_month_invoices AS (
                            SELECT COUNT(*) as invoice_count
                            FROM invoices
                            WHERE strftime('%Y-%m', created_at) = strftime('%Y-%m', NEW.created_at)
                            AND id <= NEW.id
                        )
                        SELECT format(
                            'F-%s-%03d',
                            SUBSTRING(strftime('%Y-%m', NEW.created_at), 3),
                            invoice_count
                        )
                        FROM current_month_invoices
                    )
                    WHERE id = NEW.id;
                END;
            "#,
        );
        db.execute_raw(i_identifier_generator).await?;

        let o_identifier_generator = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
                CREATE TRIGGER IF NOT EXISTS order_identifier_generator
                AFTER INSERT ON orders 
                BEGIN
                    UPDATE orders 
                    SET identifier = (
                        WITH current_month_orders AS (
                            SELECT COUNT(*) as order_count
                            FROM orders
                            WHERE strftime('%Y-%m', created_at) = strftime('%Y-%m', NEW.created_at)
                            AND id <= NEW.id
                        )
                        SELECT format(
                            'C-%s-%03d',
                            SUBSTRING(strftime('%Y-%m', NEW.created_at), 3),
                            order_count
                        )
                        FROM current_month_orders
                    )
                    WHERE id = NEW.id;
                END;
            "#,
        );
        db.execute_raw(o_identifier_generator).await?;

        let q_identifier_generator = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
                CREATE TRIGGER IF NOT EXISTS quote_identifier_generator
                AFTER INSERT ON quotes 
                BEGIN
                    UPDATE quotes 
                    SET identifier = (
                        WITH current_month_quotes AS (
                            SELECT COUNT(*) as quote_count
                            FROM quotes
                            WHERE strftime('%Y-%m', created_at) = strftime('%Y-%m', NEW.created_at)
                            AND id <= NEW.id
                        )
                        SELECT format(
                            'D-%s-%03d',
                            SUBSTRING(strftime('%Y-%m', NEW.created_at), 3),
                            quote_count
                        )
                        FROM current_month_quotes
                    )
                    WHERE id = NEW.id;
                END;
            "#,
        );
        db.execute_raw(q_identifier_generator).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TRIGGER IF EXISTS invoice_identifier_generator")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS order_identifier_generator")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS quote_identifier_generator")
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(Client::Table)
                    .name("idx_clients_fullname")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Supplier::Table)
                    .name("idx_suppliers_fullname")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Product::Table)
                    .name("idx_products_name")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Quote::Table)
                    .name("idx_quote_client_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(QuoteItem::Table)
                    .name("idx_quote_item_product_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(OrderItem::Table)
                    .name("idx_order_item_inventory_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Order::Table)
                    .name("idx_order_client_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Invoice::Table)
                    .name("idx_invoice_client_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(InventoryTransaction::Table)
                    .name("idx_inventory_product_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(InventoryTransaction::Table)
                    .name("idx_inventory_transaction_type")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Order::Table)
                    .name("idx_orders_status")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Invoice::Table)
                    .name("idx_invoices_status")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(InvoiceItem::Table)
                    .name("idx_invoice_items_invoice_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(InvoiceItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Invoice::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuoteItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(InventoryTransaction::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Supplier::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Client {
    #[sea_orm(iden = "clients")]
    Table,
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
    #[sea_orm(iden = "is_archived")]
    IsArchived,
}

#[derive(DeriveIden)]
pub enum Supplier {
    #[sea_orm(iden = "suppliers")]
    Table,
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
    #[sea_orm(iden = "is_archived")]
    IsArchived,
}

#[derive(DeriveIden)]
pub enum Product {
    #[sea_orm(iden = "products")]
    Table,
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
    #[sea_orm(iden = "is_archived")]
    IsArchived,
}

#[derive(DeriveIden)]
pub enum InventoryTransaction {
    #[sea_orm(iden = "inventory_transactions")]
    Table,
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

#[derive(DeriveIden)]
pub enum Quote {
    #[sea_orm(iden = "quotes")]
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "is_archived")]
    IsArchived,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
pub enum QuoteItem {
    #[sea_orm(iden = "quote_items")]
    Table,
    Id,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "quote_id")]
    QuoteId,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "quantity")]
    Quantity,
}

#[derive(DeriveIden)]
pub enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "quote_id")]
    QuoteId,
    // status: delivered, cancel, ongoing
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "is_archived")]
    IsArchived,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
pub enum OrderItem {
    #[sea_orm(iden = "order_items")]
    Table,
    Id,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
    #[sea_orm(iden = "price")]
    Price,
}

#[derive(DeriveIden)]
pub enum Invoice {
    #[sea_orm(iden = "invoices")]
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "order_id")]
    OrderId,
    // status: paid, cancel, ongoing
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "paid_amount")]
    PaidAmount,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "is_archived")]
    IsArchived,
    #[sea_orm(iden = "identifier")]
    Identifier,
}

#[derive(DeriveIden)]
pub enum InvoiceItem {
    #[sea_orm(iden = "invoice_items")]
    Table,
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
