use mongodb::{bson::{DateTime, Uuid}, options::ClientOptions, results::InsertOneResult, Client, Database};

use log::{info, error};
use serde::{Deserialize, Serialize};

use crate::domain::certificate::Certificate;

pub struct CrsState {
    pub db: Option<Database>,
}

pub async fn init_db() -> Option<Database> {
    info!("Initializing CRS DB!");

    if let Ok(_uri) =  dotenvy::var("DB_CONN_STRING") {
        let mut client_opts = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
        client_opts.app_name = Some("CRS".to_string());
        match Client::with_options(client_opts) {
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

pub async fn store_one(db: &Database, doc: &CertificateModel) -> Option<InsertOneResult> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll.insert_one(doc, None).await {
        Ok(insert_one_result) => Some(insert_one_result),
        Err(_) => None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateModel {
    pub certificate_id: Uuid,
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
            certificate_id: Uuid::default(),
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