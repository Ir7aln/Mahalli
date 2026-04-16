use system_migration::{Migrator, MigratorTrait};
use system_service::sea_orm::{Database, DatabaseConnection};

use crate::db::paths::{sqlite_url_from_path, AppPaths};

pub async fn setup_system_database(paths: &AppPaths) -> DatabaseConnection {
    let db_url = sqlite_url_from_path(&paths.system_db_path);
    let db_conn = Database::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db_url));

    Migrator::up(&db_conn, None)
        .await
        .expect("unable to run system migrations");

    db_conn
}
