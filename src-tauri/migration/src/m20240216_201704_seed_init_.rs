use std::ops::Range;

use crate::{
    m20220101_000001_init_::{
        Client, InventoryTransaction, Invoice, InvoiceItem, Order, OrderItem, Product, Quote, QuoteItem,
        Supplier,
    },
    utils::get_random_enum,
};
use fake::{
    faker::{
        address::en::SecondaryAddress,
        internet::en::FreeEmail,
        lorem::en::{Sentence, Word},
        name::en::Name,
        phone_number::en::PhoneNumber,
    },
    Fake, Faker,
};
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 0..200 {
            let id = ulid::Ulid::new();
            let fullname: String = Name().fake();
            let address: String = SecondaryAddress().fake();
            let email: String = FreeEmail().fake();
            let phone: String = PhoneNumber().fake();
            let insert = Query::insert()
                .into_table(Client::Table)
                .columns([
                    Client::Id,
                    Client::Fullname,
                    Client::Address,
                    Client::Email,
                    Client::Phone,
                ])
                .values_panic([
                    id.to_string().into(),
                    fullname.into(),
                    address.into(),
                    email.into(),
                    phone.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        for _ in 0..200 {
            let id = ulid::Ulid::new();
            let fullname: String = Name().fake();
            let address: String = SecondaryAddress().fake();
            let email: String = FreeEmail().fake();
            let phone: String = PhoneNumber().fake();
            let insert = Query::insert()
                .into_table(Supplier::Table)
                .columns([
                    Supplier::Id,
                    Supplier::Fullname,
                    Supplier::Address,
                    Supplier::Email,
                    Supplier::Phone,
                ])
                .values_panic([
                    id.to_string().into(),
                    fullname.into(),
                    address.into(),
                    email.into(),
                    phone.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        for _ in 0..400 {
            let id = ulid::Ulid::new();
            let name: String = Word().fake();
            let rand: u8 = Faker.fake();
            let address: String = Sentence(Range { start: 5, end: 10 }).fake();
            let purchase_price = (50..150).fake::<u8>();
            let selling_price = (150..250).fake::<u8>();
            let quantity: u8 = Faker.fake();
            let insert = Query::insert()
                .into_table(Product::Table)
                .columns([
                    Product::Id,
                    Product::Name,
                    Product::Description,
                    Product::PurchasePrice,
                    Product::SellingPrice,
                    Product::MinQuantity,
                ])
                .values_panic([
                    id.to_string().into(),
                    format!("{}-{}", name, rand).into(),
                    address.into(),
                    purchase_price.into(),
                    selling_price.into(),
                    quantity.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;

            let inventory_id = ulid::Ulid::new();
            let inventory_quantity: u8 = Faker.fake();
            let insert_stock = Query::insert()
                .into_table(InventoryTransaction::Table)
                .columns([
                    InventoryTransaction::Id,
                    InventoryTransaction::ProductId,
                    InventoryTransaction::Quantity,
                    InventoryTransaction::TransactionType,
                ])
                .values_panic([
                    inventory_id.to_string().into(),
                    id.to_string().into(),
                    inventory_quantity.into(),
                    String::from("IN").into(),
                ])
                .to_owned();

            manager.exec_stmt(insert_stock).await?;
        }

        let db = manager.get_connection();

        let status = vec![
            String::from("PENDING"),
            String::from("PROCESSING"),
            String::from("SHIPPED"),
            String::from("DELIVERED"),
            String::from("CANCELLED"),
        ];

        for _ in 0..100 {
            let id = ulid::Ulid::new();
            let status_ = get_random_enum(status.clone());
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    orders (id, status, client_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into(), status_.into()],
            );
            db.execute(insert_order).await?;
        }

        for _ in 0..1000 {
            let _id = ulid::Ulid::new();
            let quantity: u8 = Faker.fake();
            let insert_inventory = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    inventory_transactions (id, transaction_type, quantity, product_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        $3,
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [
                    _id.to_string().into(),
                    String::from("OUT").into(),
                    quantity.into(),
                ],
            );
            db.execute(insert_inventory).await?;
            //
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    order_items (id, price, order_id, inventory_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM orders ORDER BY RANDOM() LIMIT 1),
                        $3
                    )
                "#,
                [id.to_string().into(), price.into(), _id.to_string().into()],
            );
            db.execute(insert_order).await?;
        }

        let status = vec![
            String::from("DRAFT"),
            String::from("SENT"),
            String::from("PAID"),
            String::from("PARTIALLY_PAID"),
            String::from("OVERDUE"),
            String::from("CANCELLED"),
        ];

        for _ in 0..100 {
            let id = ulid::Ulid::new();
            let status_ = get_random_enum(status.clone());
            let paid: u8 = Faker.fake();
            let insert_invoice = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    invoices (id, status, client_id, paid_amount, order_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1),
                        $3,
                        (SELECT id FROM orders ORDER BY RANDOM() LIMIT 1)

                    )
                ON CONFLICT DO NOTHING
                "#,
                [id.to_string().into(), status_.into(), paid.into()],
            );
            db.execute(insert_invoice).await?;
        }

        let fix_client_id = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"UPDATE invoices SET client_id = (SELECT client_id FROM orders WHERE id = order_id);"#,
        );

        db.execute(fix_client_id).await?;

        // Seed invoice_items for existing invoices
        for _ in 0..300 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_item = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO
                    invoice_items (id, invoice_id, product_id, price, quantity, inventory_id)
                VALUES
                    (
                        $1,
                        (SELECT id FROM invoices ORDER BY RANDOM() LIMIT 1),
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1),
                        $2,
                        $3,
                        (SELECT id FROM inventory_transactions ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into(), price.into(), quantity.into()],
            );
            db.execute(insert_item).await?;
        }

        for _ in 0..150 {
            let id = ulid::Ulid::new();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    quotes (id, client_id)
                VALUES
                    (
                        $1, 
                        (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into()],
            );
            db.execute(insert_quote).await?;
        }

        for _ in 0..1000 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    quote_items (id, price, product_id, quote_id, quantity)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1),
                        (SELECT id FROM quotes ORDER BY RANDOM() LIMIT 1),
                        $3
                    )
                "#,
                [id.to_string().into(), price.into(), quantity.into()],
            );
            db.execute(insert_quote).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_item = Query::delete()
            .from_table(InvoiceItem::Table)
            .to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete_item = Query::delete().from_table(OrderItem::Table).to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete_item: DeleteStatement = Query::delete().from_table(QuoteItem::Table).to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete = Query::delete().from_table(Order::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Invoice::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Quote::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete()
            .from_table(InventoryTransaction::Table)
            .to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Product::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Client::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Supplier::Table).to_owned();
        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
