use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Enable WAL mode for better concurrency
        db.execute_raw(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"PRAGMA journal_mode=WAL;"#,
        ))
        .await?;

        // Create indexes for query performance
        manager
            .create_index(
                Index::create()
                    .table(Quote::Table)
                    .col(Quote::ClientId)
                    .name("idx_quote_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(QuoteItem::Table)
                    .col(QuoteItem::ProductId)
                    .name("idx_quote_item_product_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(OrderItem::Table)
                    .col(OrderItem::InventoryId)
                    .name("idx_order_item_inventory_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Order::Table)
                    .col(Order::ClientId)
                    .name("idx_order_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Order::Table)
                    .col(Order::Status)
                    .name("idx_orders_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Invoice::Table)
                    .col(Invoice::ClientId)
                    .name("idx_invoice_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Invoice::Table)
                    .col(Invoice::Status)
                    .name("idx_invoices_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(InvoicePayment::Table)
                    .col(InvoicePayment::InvoiceId)
                    .name("idx_invoice_payments_invoice_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(InvoicePayment::Table)
                    .col(InvoicePayment::PaymentDate)
                    .name("idx_invoice_payments_payment_date")
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
                Index::create()
                    .table(InventoryTransaction::Table)
                    .col(InventoryTransaction::ProductId)
                    .name("idx_inventory_product_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(InventoryTransaction::Table)
                    .col(InventoryTransaction::TransactionType)
                    .name("idx_inventory_transaction_type")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(DeliveryNote::Table)
                    .col(DeliveryNote::OrderId)
                    .name("idx_delivery_notes_order_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(DeliveryNote::Table)
                    .col(DeliveryNote::ClientId)
                    .name("idx_delivery_notes_client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(DeliveryNoteItem::Table)
                    .col(DeliveryNoteItem::DeliveryNoteId)
                    .name("idx_delivery_note_items_delivery_note_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Client::Table)
                    .col(Client::Fullname)
                    .name("idx_clients_fullname")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Product::Table)
                    .col(Product::Name)
                    .name("idx_products_name")
                    .to_owned(),
            )
            .await?;

        // Create triggers for auto-generated identifiers
        let invoice_trigger = Statement::from_string(
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
        db.execute_raw(invoice_trigger).await?;

        let order_trigger = Statement::from_string(
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
        db.execute_raw(order_trigger).await?;

        let quote_trigger = Statement::from_string(
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
        db.execute_raw(quote_trigger).await?;

        let delivery_note_trigger = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
                CREATE TRIGGER IF NOT EXISTS delivery_note_identifier_generator
                AFTER INSERT ON delivery_notes
                BEGIN
                    UPDATE delivery_notes
                    SET identifier = (
                        WITH current_month_delivery_notes AS (
                            SELECT COUNT(*) as delivery_note_count
                            FROM delivery_notes
                            WHERE strftime('%Y-%m', created_at) = strftime('%Y-%m', NEW.created_at)
                            AND id <= NEW.id
                        )
                        SELECT format(
                            'BL-%s-%03d',
                            SUBSTRING(strftime('%Y-%m', NEW.created_at), 3),
                            delivery_note_count
                        )
                        FROM current_month_delivery_notes
                    )
                    WHERE id = NEW.id;
                END;
            "#,
        );
        db.execute_raw(delivery_note_trigger).await?;

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
        db.execute_unprepared("DROP TRIGGER IF EXISTS delivery_note_identifier_generator")
            .await?;

        manager
            .drop_index(Index::drop().table(Quote::Table).name("idx_quote_client_id").to_owned())
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
            .drop_index(Index::drop().table(Order::Table).name("idx_order_client_id").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().table(Order::Table).name("idx_orders_status").to_owned())
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
                    .table(Invoice::Table)
                    .name("idx_invoices_status")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(InvoicePayment::Table)
                    .name("idx_invoice_payments_invoice_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(InvoicePayment::Table)
                    .name("idx_invoice_payments_payment_date")
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
                    .table(DeliveryNote::Table)
                    .name("idx_delivery_notes_order_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(DeliveryNote::Table)
                    .name("idx_delivery_notes_client_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(DeliveryNoteItem::Table)
                    .name("idx_delivery_note_items_delivery_note_id")
                    .to_owned(),
            )
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
                    .table(Product::Table)
                    .name("idx_products_name")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Quote {
    #[sea_orm(iden = "quotes")]
    Table,
    #[sea_orm(iden = "client_id")]
    ClientId,
}

#[derive(DeriveIden)]
enum QuoteItem {
    #[sea_orm(iden = "quote_items")]
    Table,
    #[sea_orm(iden = "product_id")]
    ProductId,
}

#[derive(DeriveIden)]
enum OrderItem {
    #[sea_orm(iden = "order_items")]
    Table,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
}

#[derive(DeriveIden)]
enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "status")]
    Status,
}

#[derive(DeriveIden)]
enum Invoice {
    #[sea_orm(iden = "invoices")]
    Table,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "status")]
    Status,
}

#[derive(DeriveIden)]
enum InvoicePayment {
    #[sea_orm(iden = "invoice_payments")]
    Table,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
    #[sea_orm(iden = "payment_date")]
    PaymentDate,
}

#[derive(DeriveIden)]
enum InvoiceItem {
    #[sea_orm(iden = "invoice_items")]
    Table,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
}

#[derive(DeriveIden)]
enum InventoryTransaction {
    #[sea_orm(iden = "inventory_transactions")]
    Table,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "transaction_type")]
    TransactionType,
}

#[derive(DeriveIden)]
enum DeliveryNote {
    #[sea_orm(iden = "delivery_notes")]
    Table,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "client_id")]
    ClientId,
}

#[derive(DeriveIden)]
enum DeliveryNoteItem {
    #[sea_orm(iden = "delivery_note_items")]
    Table,
    #[sea_orm(iden = "delivery_note_id")]
    DeliveryNoteId,
}

#[derive(DeriveIden)]
enum Client {
    #[sea_orm(iden = "clients")]
    Table,
    #[sea_orm(iden = "full_name")]
    Fullname,
}

#[derive(DeriveIden)]
enum Product {
    #[sea_orm(iden = "products")]
    Table,
    #[sea_orm(iden = "name")]
    Name,
}
