use apalis::{
    layers::tracing::TraceLayer, prelude::*, sqlite::SqliteStorage, utils::TokioExecutor,
};

use db::setup_database;
use jobs::{process_image, setup_jobs_database, ImageOptimizerJobStorage, ImageProcessorJob};
use service::sea_orm::DatabaseConnection;

mod commands;
mod db;
mod jobs;

pub struct AppState {
    db_conn: DatabaseConnection,
    job_storage: ImageOptimizerJobStorage,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // db
    let db_conn = setup_database().await;
    // jobs
    let pool = setup_jobs_database().await;

    let image_storage: SqliteStorage<ImageProcessorJob> = SqliteStorage::new(pool.clone());
    let thread_safe_storage = ImageOptimizerJobStorage::new(image_storage.clone());

    let monitor = Monitor::<TokioExecutor>::new().register_with_count(2, {
        WorkerBuilder::new("image-processor")
            .layer(TraceLayer::new())
            .data(db_conn.clone())
            .with_storage(image_storage)
            .build_fn(process_image)
    });

    tokio::spawn(async move {
        monitor.run().await.unwrap();
    });

    tauri::Builder::default()
        .manage(AppState {
            db_conn,
            job_storage: thread_safe_storage,
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .level_for("tauri", log::LevelFilter::Error)
                .level_for("hyper", log::LevelFilter::Off)
                .level_for("tracing", log::LevelFilter::Info)
                .level_for("sea_orm", log::LevelFilter::Info)
                .level_for("sqlx", log::LevelFilter::Off)
                .level_for("tao", log::LevelFilter::Off)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            //
            // products
            //
            commands::products::list_products,
            commands::products::search_products,
            commands::products::create_product,
            commands::products::update_product,
            commands::products::delete_product,
            //
            // inventory
            //
            commands::inventory::list_inventory,
            commands::inventory::create_inventory,
            commands::inventory::delete_inventory,
            //
            // clients
            //
            commands::clients::list_clients,
            commands::clients::search_clients,
            commands::clients::get_client,
            commands::clients::create_client,
            commands::clients::update_client,
            commands::clients::delete_client,
            //
            // suppliers
            //
            commands::suppliers::list_suppliers,
            commands::suppliers::search_suppliers,
            commands::suppliers::create_supplier,
            commands::suppliers::update_supplier,
            commands::suppliers::delete_supplier,
            //
            // orders
            //
            commands::orders::list_orders,
            commands::orders::get_order,
            commands::orders::get_order_details,
            commands::orders::create_order,
            commands::orders::update_order,
            commands::orders::delete_order,
            commands::orders::list_order_products,
            commands::orders::create_order_from_quote,
            commands::orders::update_order_status,
            //
            // quotes
            //
            commands::quotes::list_quotes,
            commands::quotes::get_quote,
            commands::quotes::get_quote_details,
            commands::quotes::create_quote,
            commands::quotes::update_quote,
            commands::quotes::delete_quote,
            commands::quotes::list_quote_products,
            //
            // quote items
            //
            commands::templates::create_template,
            //
            // quote items
            //
            commands::quote_items::delete_quote_item,
            //
            // invoices
            //
            commands::invoices::list_invoices,
            commands::invoices::get_invoice,
            commands::invoices::get_invoice_details,
            commands::invoices::create_invoice,
            commands::invoices::update_invoice,
            commands::invoices::delete_invoice,
            commands::invoices::list_invoice_products,
            commands::invoices::create_invoice_from_order,
            commands::invoices::update_invoice_status,
            //
            // dashboard
            //
            commands::dashboard::list_inventory_stats,
            commands::dashboard::list_top_clients,
            commands::dashboard::list_top_suppliers,
            commands::dashboard::list_top_products,
            commands::dashboard::list_status_count,
            commands::dashboard::list_financial_metrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
