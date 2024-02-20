use mongodb::{bson::{DateTime, Uuid}, Client, Database};

use log::{info, error};
use serde::{Deserialize, Serialize};

use crate::domain::certificate::Certificate;

pub struct CrsState {
    pub db: Option<Database>,
}

pub async fn init_db() -> Option<Database> {
    info!("Initializing CRS DB!");

    if let Ok(uri) =  dotenvy::var("DB_CONN_STRING") {
        match Client::with_uri_str(uri).await {
            Ok(client) => {
                let db = client.database("crs");
                return Some(db);
            },
            Err(_) => error!("Unable to initialize DB")
        }
    }
    
    error!("DB connection string is invalid!");
    None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub product_id: u32,
    pub metadata: CertificateMetadataModel,
    pub created_date: DateTime,
    pub updated_date: DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateMetadataModel {
    pub score: u32,
    pub progress: u32,
    pub pe_points: u32,
    pub acquired_date: DateTime,
}

impl CertificateModel {
    pub fn convert(certificate: &Certificate) -> CertificateModel {
        CertificateModel {
            id: Uuid::default(),
            user_id: Uuid::from_uuid_1(certificate.user_id),
            account_id: Uuid::from_uuid_1(certificate.account_id),
            product_id: certificate.product_id,
            metadata: CertificateMetadataModel {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                pe_points: certificate.metadata.pe_points,
                acquired_date: DateTime::from_chrono(certificate.metadata.acquired_date),
            },
            created_date: DateTime::now(),
            updated_date: DateTime::MIN
        }
    }
}