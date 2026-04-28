use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct AppPaths {
    pub root_dir: PathBuf,
    pub system_dir: PathBuf,
    pub tenants_dir: PathBuf,
    pub system_db_path: PathBuf,
    pub default_tenant_path: PathBuf,
}

impl AppPaths {
    pub fn resolve() -> Self {
        #[cfg(debug_assertions)]
        {
            dotenvy::dotenv().ok();
        }

        let root_dir = resolve_root_dir();
        let system_dir = root_dir.join("system");
        let tenants_dir = root_dir.join("tenants");
        let system_db_path = system_dir.join("catalog.sqlite");
        let default_tenant_path = resolve_default_tenant_path(&tenants_dir);

        std::fs::create_dir_all(&system_dir).expect("Could not create system directory");
        std::fs::create_dir_all(&tenants_dir).expect("Could not create tenants directory");

        Self {
            root_dir,
            system_dir,
            tenants_dir,
            system_db_path,
            default_tenant_path,
        }
    }
}

fn resolve_root_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            if let Some(path) = sqlite_url_to_path(&database_url) {
                if let Some(parent) = path.parent() {
                    return parent.to_path_buf();
                }
            }
        }
    }

    let home_dir = dirs::data_dir().unwrap_or_else(|| panic!("Could not get home directory"));
    home_dir.join(".mahalli").join("data")
}

fn resolve_default_tenant_path(tenants_dir: &Path) -> PathBuf {
    #[cfg(debug_assertions)]
    {
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            if let Some(path) = sqlite_url_to_path(&database_url) {
                return path;
            }
        }
    }

    tenants_dir.join("main.sqlite")
}

fn sqlite_url_to_path(url: &str) -> Option<PathBuf> {
    let path = url.strip_prefix("sqlite://")?;
    let path = path.split('?').next()?;
    Some(PathBuf::from(path))
}

pub fn sqlite_url_from_path(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{normalized}?mode=rwc")
}
