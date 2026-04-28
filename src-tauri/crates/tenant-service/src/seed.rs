use std::ops::Range;

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
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};

fn get_random_item<T: Clone>(items: &[T]) -> T {
    let idx = (Faker.fake::<u8>() as usize) % items.len();
    items[idx].clone()
}

pub struct SeedService;

impl SeedService {
    pub async fn seed_database(db: &DatabaseConnection) -> Result<(), DbErr> {
        Self::seed_clients(db).await?;
        Self::seed_products(db).await?;
        Self::seed_quotes(db).await?;
        Self::seed_quote_items(db).await?;
        Self::seed_orders(db).await?;
        Self::seed_order_items(db).await?;
        Self::seed_delivery_notes(db).await?;
        Self::seed_delivery_note_items(db).await?;
        Self::seed_invoices(db).await?;
        Self::seed_invoice_items(db).await?;
        Self::seed_invoice_payments(db).await?;
        Self::seed_credit_notes(db).await?;
        Self::seed_credit_note_items(db).await?;
        Ok(())
    }

    async fn seed_clients(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..200 {
            let id = ulid::Ulid::new();
            let full_name: String = Name().fake();
            let phone_number: String = PhoneNumber().fake();
            let email: String = FreeEmail().fake();
            let address: String = SecondaryAddress().fake();

            let insert = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO clients (id, full_name, is_deleted, phone_number, email, address) VALUES ($1, $2, $3, $4, $5, $6)"#,
                [
                    id.to_string().into(),
                    full_name.into(),
                    false.into(),
                    phone_number.into(),
                    email.into(),
                    address.into(),
                ],
            );

            db.execute_raw(insert).await?;
        }
        Ok(())
    }

    async fn seed_products(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..400 {
            let id = ulid::Ulid::new();
            let name: String = Word().fake();
            let rand: u8 = Faker.fake();
            let description: String = Sentence(Range { start: 5, end: 10 }).fake();
            let purchase_price = (50..150).fake::<u8>();
            let selling_price = (150..250).fake::<u8>();
            let quantity: u8 = Faker.fake();

            let insert = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO products (id, name, is_deleted, description, purchase_price, selling_price, min_quantity) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                [
                    id.to_string().into(),
                    format!("{}-{}", name, rand).into(),
                    false.into(),
                    description.into(),
                    (purchase_price as f32).into(),
                    (selling_price as f32).into(),
                    (quantity as f32).into(),
                ],
            );

            db.execute_raw(insert).await?;

            let inventory_id = ulid::Ulid::new();
            let inventory_quantity: u8 = Faker.fake();
            let insert_stock = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO inventory_transactions (id, product_id, quantity, transaction_type) VALUES ($1, $2, $3, $4)"#,
                [
                    inventory_id.to_string().into(),
                    id.to_string().into(),
                    (inventory_quantity as f32).into(),
                    String::from("IN").into(),
                ],
            );

            db.execute_raw(insert_stock).await?;
        }
        Ok(())
    }

    async fn seed_orders(db: &DatabaseConnection) -> Result<(), DbErr> {
        let statuses = vec!["PENDING", "COMPLETED", "CANCELLED"];

        for _ in 0..100 {
            let id = ulid::Ulid::new();
            let status = get_random_item(&statuses);
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO orders (id, status, client_id) VALUES ($1, $2, (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1))"#,
                [id.to_string().into(), status.to_string().into()],
            );
            db.execute_raw(insert_order).await?;
        }
        Ok(())
    }

    async fn seed_order_items(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..1000 {
            let _id = ulid::Ulid::new();
            let quantity: u8 = Faker.fake();
            let insert_inventory = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO inventory_transactions (id, transaction_type, quantity, product_id) VALUES ($1, $2, $3, (SELECT id FROM products ORDER BY RANDOM() LIMIT 1))"#,
                [
                    _id.to_string().into(),
                    String::from("OUT").into(),
                    (quantity as f32).into(),
                ],
            );
            db.execute_raw(insert_inventory).await?;

            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO order_items (id, price, order_id, inventory_id) VALUES ($1, $2, (SELECT id FROM orders ORDER BY RANDOM() LIMIT 1), $3)"#,
                [
                    id.to_string().into(),
                    (price as f32).into(),
                    _id.to_string().into(),
                ],
            );
            db.execute_raw(insert_order).await?;
        }
        Ok(())
    }

    async fn seed_invoices(db: &DatabaseConnection) -> Result<(), DbErr> {
        let statuses = vec!["DRAFT", "PAID", "PARTIALLY_PAID", "CANCELLED"];

        for _ in 0..100 {
            let id = ulid::Ulid::new();
            let status = get_random_item(&statuses);
            let insert_invoice = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO invoices (id, status, client_id, order_id) VALUES ($1, $2, (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1), (SELECT id FROM orders ORDER BY RANDOM() LIMIT 1)) ON CONFLICT DO NOTHING"#,
                [id.to_string().into(), status.to_string().into()],
            );
            db.execute_raw(insert_invoice).await?;
        }

        let fix_client_id = Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "UPDATE invoices SET client_id = (SELECT client_id FROM orders WHERE id = order_id);"
                .to_string(),
        );

        db.execute_raw(fix_client_id).await?;

        Ok(())
    }

    async fn seed_invoice_items(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..300 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_item = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO invoice_items (id, invoice_id, product_id, price, quantity, inventory_id) VALUES ($1, (SELECT id FROM invoices ORDER BY RANDOM() LIMIT 1), (SELECT id FROM products ORDER BY RANDOM() LIMIT 1), $2, $3, (SELECT id FROM inventory_transactions ORDER BY RANDOM() LIMIT 1))"#,
                [
                    id.to_string().into(),
                    (price as f32).into(),
                    (quantity as f32).into(),
                ],
            );
            db.execute_raw(insert_item).await?;
        }
        Ok(())
    }

    async fn seed_invoice_payments(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..150 {
            let id = ulid::Ulid::new();
            let amount: u8 = (10..100).fake();
            let insert_payment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                    INSERT INTO invoice_payments (id, invoice_id, payment_date, amount, description)
                    SELECT
                        $1,
                        invoices.id,
                        CURRENT_TIMESTAMP,
                        MIN($2, COALESCE(invoice_totals.total_amount, 0)),
                        $3
                    FROM invoices
                    LEFT JOIN (
                        SELECT invoice_id, SUM(price * quantity) AS total_amount
                        FROM invoice_items
                        GROUP BY invoice_id
                    ) AS invoice_totals ON invoice_totals.invoice_id = invoices.id
                    ORDER BY RANDOM()
                    LIMIT 1
                "#,
                [
                    id.to_string().into(),
                    (amount as f64).into(),
                    String::from("Seed payment").into(),
                ],
            );
            db.execute_raw(insert_payment).await?;
        }
        Ok(())
    }

    async fn seed_quotes(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..150 {
            let id = ulid::Ulid::new();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO quotes (id, client_id) VALUES ($1, (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1))"#,
                [id.to_string().into()],
            );
            db.execute_raw(insert_quote).await?;
        }
        Ok(())
    }

    async fn seed_quote_items(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..1000 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"INSERT INTO quote_items (id, price, product_id, quote_id, quantity) VALUES ($1, $2, (SELECT id FROM products ORDER BY RANDOM() LIMIT 1), (SELECT id FROM quotes ORDER BY RANDOM() LIMIT 1), $3)"#,
                [
                    id.to_string().into(),
                    (price as f32).into(),
                    (quantity as f32).into(),
                ],
            );
            db.execute_raw(insert_quote).await?;
        }
        Ok(())
    }

    async fn seed_delivery_notes(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..80 {
            let id = ulid::Ulid::new();
            let insert_delivery_note = Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO delivery_notes (id, order_id, client_id, is_deleted) SELECT '{}', id, client_id, 0 FROM orders ORDER BY RANDOM() LIMIT 1",
                    id
                ),
            );
            db.execute_raw(insert_delivery_note).await?;
        }
        Ok(())
    }

    async fn seed_delivery_note_items(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..250 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_item = Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO delivery_note_items (id, delivery_note_id, product_id, price, quantity) SELECT '{}', dn.id, p.id, {}, {} FROM delivery_notes dn, products p ORDER BY RANDOM() LIMIT 1",
                    id, price as f32, quantity as f32
                ),
            );
            db.execute_raw(insert_item).await?;
        }
        Ok(())
    }

    async fn seed_credit_notes(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..40 {
            let id = ulid::Ulid::new();
            let reasons = vec!["Defective product", "Wrong item shipped", "Quantity mismatch", "Customer request"];
            let reason = get_random_item(&reasons);
            let insert_credit_note = Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO credit_notes (id, invoice_id, client_id, is_deleted, reason) SELECT '{}', id, client_id, 0, '{}' FROM invoices ORDER BY RANDOM() LIMIT 1",
                    id, reason
                ),
            );
            db.execute_raw(insert_credit_note).await?;
        }
        Ok(())
    }

    async fn seed_credit_note_items(db: &DatabaseConnection) -> Result<(), DbErr> {
        for _ in 0..120 {
            let id = ulid::Ulid::new();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert_item = Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO credit_note_items (id, credit_note_id, product_id, price, quantity) SELECT '{}', cn.id, p.id, {}, {} FROM credit_notes cn, products p ORDER BY RANDOM() LIMIT 1",
                    id, price as f32, quantity as f32
                ),
            );
            db.execute_raw(insert_item).await?;
        }
        Ok(())
    }
}
