use apalis::{
    layers::tracing::TraceLayer, prelude::*, sqlite::SqliteStorage, utils::TokioExecutor,
};
use db::setup_database;
use jobs::{process_image, setup_jobs_database, ImageOptimizerJobStorage, ImageProcessorJob};
use service::sea_orm::DatabaseConnection;

mod commands;
mod db;
mod jobs;
mod specta;

pub struct AppState {
    db_conn: DatabaseConnection,
    job_storage: ImageOptimizerJobStorage,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // Export bindings only in debug mode
    #[cfg(debug_assertions)]
    specta::export_bindings();

    // Database setup
    let db_conn = setup_database().await;

    // Background jobs setup
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

    // Run job monitor in background
    tokio::spawn(async move {
        monitor.run().await.unwrap();
    });

    // Build the Specta handler
    let specta_builder = specta::builder::<tauri::Wry>();

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
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                }))
                .level_for("tauri", log::LevelFilter::Error)
                .level_for("hyper", log::LevelFilter::Off)
                .level_for("tracing", log::LevelFilter::Info)
                .level_for("sea_orm", log::LevelFilter::Info)
                .level_for("sqlx", log::LevelFilter::Off)
                .level_for("tao", log::LevelFilter::Off)
                .build(),
        )
        // This is the correct way in tauri-specta v2 rc.24
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            // If you use events later, uncomment this:
            specta_builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
