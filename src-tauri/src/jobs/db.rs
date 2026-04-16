use apalis::sqlite::SqliteStorage;
use sqlx::SqlitePool;

pub async fn setup_jobs_database() -> SqlitePool {
    let db_url = get_database_url();
    let sqlite_pool = SqlitePool::connect(&db_url)
        .await
        .expect(&format!("Error getting sqlitepool to {}", &db_url));
    SqliteStorage::setup(&sqlite_pool)
        .await
        .expect("unable to run jobs migrations");

    return sqlite_pool;
}

fn get_database_url() -> String {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
        std::env::var("JOBS_DATABASE_URL").unwrap()
    }

    #[cfg(not(debug_assertions))]
    {
        let home_dir = dirs::data_dir().unwrap_or_else(|| panic!("Could not get home directory"));
        let data_dir = home_dir.join(".mahalli/data");
        std::fs::create_dir_all(&data_dir)
            .unwrap_or_else(|_| panic!("Could not create data directory"));
        crate::db::paths::sqlite_url_from_path(&data_dir.join("jobs.sqlite"))
    }
}
