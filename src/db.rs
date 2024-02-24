use mongodb::{
    bson::{doc, DateTime, Uuid},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Database,
};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::domain::certificate::Certificate;

pub const DB_NAME: &str = "crs";

pub struct CrsState {
    pub db: Option<Database>,
}

pub async fn init_db() -> Option<Database> {
    info!("Initializing CRS DB!");

    if let Ok(uri) = dotenvy::var("DB_CONN_STRING") {
        let mut client_opts = ClientOptions::parse(uri)
            .await
            .expect("Unable to create ClientOptions");
        client_opts.app_name = Some("CRS".to_string());
        match Client::with_options(client_opts) {
            Ok(client) => {
                let db = client.database(DB_NAME);
                return Some(db);
            }
            Err(err) => error!("Unable to initialize DB. {}", err.to_string()),
        }
    }

    error!("DB connection string is invalid!");
    None
}

pub async fn store_one(db: &Database, doc: &CertificateModel) -> Option<InsertOneResult> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll.insert_one(doc, None).await {
        Ok(insert_one_result) => Some(insert_one_result),
        Err(_) => None,
    }
}

pub async fn find_certificate_by_id(
    db: &Database,
    certificate_id: uuid::Uuid,
) -> Option<CertificateModel> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll
        .find_one(
            doc! {"certificate_id": Uuid::from_uuid_1(certificate_id)},
            None,
        )
        .await
    {
        Ok(find_one_result) => find_one_result,
        Err(_) => None,
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
    pub updated_date: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateMetadataModel {
    pub score: u32,
    pub progress: u32,
    pub pe_points: u32,
    pub acquired_date: Option<DateTime>,
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
                acquired_date: match certificate.metadata.acquired_date {
                    Some(acquired_date) => Some(DateTime::from_chrono(acquired_date)),
                    None => None,
                },
            },
            created_date: DateTime::from_chrono(certificate.created_date),
            updated_date: Some(DateTime::now()),
        }
    }
}
