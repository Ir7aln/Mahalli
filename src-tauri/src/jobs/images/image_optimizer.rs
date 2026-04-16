use std::fs;
use std::path::Path;
use std::sync::Arc;

use apalis::{prelude::*, sqlite::SqliteStorage};
use dirs;
use log::warn;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

use tenant_service::{
    sea_orm::DatabaseConnection, MutationsService, UpdateClient, UpdateProduct, UpdateSupplier,
};

use super::utils::ImageProcessor;

#[derive(Debug, Deserialize, Serialize)]
pub enum EntityEnum {
    CLIENT,
    PRODUCT,
    SUPPLIER,
}

impl ToString for EntityEnum {
    fn to_string(&self) -> String {
        match self {
            EntityEnum::CLIENT => String::from("clients"),
            EntityEnum::SUPPLIER => String::from("suppliers"),
            EntityEnum::PRODUCT => String::from("products"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageProcessorJob {
    pub id: String,
    pub entity: EntityEnum,
    pub data: String,
}

impl Job for ImageProcessorJob {
    const NAME: &'static str = "image_processor";
}

#[derive(Clone)]
pub struct ImageOptimizerJobStorage {
    storage: Arc<Mutex<SqliteStorage<ImageProcessorJob>>>,
}

impl ImageOptimizerJobStorage {
    pub fn new(storage: SqliteStorage<ImageProcessorJob>) -> Self {
        Self {
            storage: Arc::new(Mutex::new(storage)),
        }
    }

    pub async fn push_job(
        &self,
        job: ImageProcessorJob,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.lock().await;
        storage.push(job).await?;
        Ok(())
    }
}

pub async fn process_image(
    job: ImageProcessorJob,
    db_conn: Data<Arc<RwLock<Option<DatabaseConnection>>>>,
) -> Result<(), Error> {
    let processor = ImageProcessor::new(1920);
    let active_db = match db_conn.read().await.clone() {
        Some(connection) => connection,
        None => {
            warn!("skipping image job because no active tenant database is selected");
            return Ok(());
        }
    };

    let home_dir = match dirs::data_dir() {
        Some(val) => val,
        None => panic!("Could not get home directory"),
    };

    let entity = job.entity.to_string();
    let output_path = home_dir
        .join(".mahalli")
        .join("data")
        .join("images")
        .join(&entity);

    if let Err(_) = fs::metadata(&output_path) {
        fs::create_dir_all(&output_path).expect("Could not create data directory");
    }

    let format = Path::new(&job.data)
        .extension()
        .unwrap_or_else(|| "".as_ref())
        .to_str()
        .unwrap_or("png");

    let image_path = output_path.join(format!("{}.{}", job.id, format));

    let processed_cropped = processor.process_image(
        &Path::new(&job.data),
        &image_path,
        Some((100, 100, 200, 200)),
    );

    if processed_cropped.is_err() {
        warn!("processing the image didnt work");
        return Ok(());
    };

    let updated_entity = match job.entity {
        EntityEnum::CLIENT => {
            MutationsService::partial_update_client(
                &active_db,
                UpdateClient {
                    id: job.id,
                    full_name: Option::None,
                    email: Option::None,
                    address: Option::None,
                    phone_number: Option::None,
                    image: Option::Some(image_path.display().to_string()),
                },
            )
            .await
        }
        EntityEnum::SUPPLIER => {
            MutationsService::partial_update_supplier(
                &active_db,
                UpdateSupplier {
                    id: job.id,
                    full_name: Option::None,
                    email: Option::None,
                    address: Option::None,
                    phone_number: Option::None,
                    image: Option::Some(image_path.display().to_string()),
                },
            )
            .await
        }
        EntityEnum::PRODUCT => {
            MutationsService::partial_update_product(
                &active_db,
                UpdateProduct {
                    id: job.id,
                    name: Option::None,
                    purchase_price: Option::None,
                    selling_price: Option::None,
                    description: Option::None,
                    min_quantity: Option::None,
                    image: Option::Some(image_path.display().to_string()),
                },
            )
            .await
        }
    };

    if updated_entity.is_err() {
        warn!("updating {:?} didnt work", entity);
        return Ok(());
    };

    Ok(())
}
