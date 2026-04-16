use db::{manager::DatabaseManager, paths::AppPaths, system::setup_system_database};
use std::sync::Arc;
use system_service::sea_orm::DatabaseConnection as SystemDatabaseConnection;
use tenant_service::sea_orm::DatabaseConnection as TenantDatabaseConnection;
use tokio::sync::RwLock;

mod commands;
mod db;
mod specta;

pub struct AppState {
    system_db_conn: SystemDatabaseConnection,
    tenant_db_conn: Arc<RwLock<Option<TenantDatabaseConnection>>>,
    db_manager: DatabaseManager,
}

impl AppState {
    pub fn system_db(&self) -> &SystemDatabaseConnection {
        &self.system_db_conn
    }

    pub async fn tenant_db(&self) -> Result<TenantDatabaseConnection, String> {
        self.tenant_db_conn
            .read()
            .await
            .clone()
            .ok_or_else(|| String::from("No active tenant database selected"))
    }

    pub async fn set_tenant_db(&self, db_conn: Option<TenantDatabaseConnection>) {
        *self.tenant_db_conn.write().await = db_conn;
    }

    pub fn db_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // Export bindings only in debug mode
    #[cfg(debug_assertions)]
    specta::export_bindings();

    let app_paths = AppPaths::resolve();
    let db_manager = DatabaseManager::new(app_paths.clone());
    let system_db_conn = setup_system_database(&app_paths).await;
    let db_conn = db_manager
        .open_active_tenant_database(&system_db_conn)
        .await
        .expect("unable to resolve active tenant database");
    let tenant_db_conn = Arc::new(RwLock::new(db_conn));

    // Build the Specta handler
    let specta_builder = specta::builder::<tauri::Wry>();

    tauri::Builder::default()
        .manage(AppState {
            system_db_conn,
            tenant_db_conn,
            db_manager,
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
