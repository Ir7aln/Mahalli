use std::path::Path;

use tenant_migration::{Migrator, MigratorTrait};
use tenant_service::sea_orm::{Database, DatabaseConnection};

use crate::db::paths::sqlite_url_from_path;

pub async fn open_tenant_database(path: &Path) -> DatabaseConnection {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Could not create tenant database directory");
    }

    let db_url = sqlite_url_from_path(path);
    let db_conn = Database::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db_url));

    Migrator::up(&db_conn, None)
        .await
        .expect("unable to run tenant migrations");

    db_conn
}
